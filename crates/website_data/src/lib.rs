use anyhow::*;
use chrono::{DateTime, Utc};
use notion_client::{
    endpoints::{databases::query::request::QueryDatabaseRequest, Client},
    objects::page::{DateOrDateTime, DatePropertyValue, Page, PageProperty, SelectPropertyValue},
};
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{self, File},
    io::Write,
    vec,
};
use tool::{parse_file_url, parse_url_to_file_info};
use website_model::work::*;

pub mod tool;

const WORKS_DATABASE_ID: &str = "9ff4f3a98e8343acb33defe8f82804bd";

const WORKS_DATA_PATH: &str = "works/";

pub async fn run_notion_data_collection() -> Result<()> {
    let notion_api = Client::new(std::env::var("NOVA_WEBSITE_NOTION_TOKEN")?)?;

    collect_database_to_file(&notion_api, WORKS_DATABASE_ID, WORKS_DATA_PATH).await?;
    Ok(())
}

pub async fn collect_database_to_file(
    client: &Client,
    database_id: &str,
    path: &str,
) -> Result<()> {
    let works_pages = client
        .databases
        .query_a_database(database_id, QueryDatabaseRequest::default())
        .await?
        .results;

    //(url, native_position)
    let mut to_download: Vec<(String, String)> = vec![];
    let mut works_objects: Vec<Work> = vec![];
    for page in works_pages {
        works_objects.push({
            let properties = &page.properties;
            let id = page.id.to_string();
            let name = if let Some(PageProperty::Title { title, .. }) = page.properties.get("Name")
            {
                tool::get_plain_string(title)
            } else {
                String::default()
            };
            let sub_name = get_plain_text_or_none(&properties.get("SubName").unwrap());
            let introduce = get_plain_text_or_none(&properties.get("Introduce").unwrap())
                .unwrap_or("".to_string());
            let tags = get_multi_selected_or_none(&properties.get("Tag").unwrap());
            let auto_collection = get_checkbox_or_none(&properties.get("AutoCollection").unwrap());
            let gamejams = get_multi_selected_or_none(&properties.get("GameJam").unwrap());
            let nova_gamejams = get_multi_selected_or_none(&properties.get("NovaGameJam").unwrap());
            let mut platforms = Vec::<Platform>::new();
            push_platform_when_exist(&properties, "Itch", &mut platforms, PlatformType::Itch);
            push_platform_when_exist(&properties, "Steam", &mut platforms, PlatformType::Steam);
            push_platform_when_exist(&properties, "GameCore", &mut platforms, PlatformType::GameCore);
            push_platform_when_exist(&properties, "HomePage", &mut platforms, PlatformType::HomePage);

            let authors = parse_authors(
                &properties.get("Author").unwrap(),
                &properties.get("AuthorLink").unwrap(),
            );
            let submission_date = if let PageProperty::Date { // standard format with date and time
                date:
                    Some(DatePropertyValue {
                        start: Some(DateOrDateTime::DateTime(it)),
                        ..
                    }),
                ..
            } = properties.get("SubmissionDate").unwrap()
            {
                it.clone().to_rfc3339()
            } else if let PageProperty::Date { // date only
                date:
                    Some(DatePropertyValue {
                        start: Some(DateOrDateTime::Date(it)),
                        ..
                    }),
                ..
            } = properties.get("SubmissionDate").unwrap()
            {
                format!("{}T00:00:00+00:00", it.clone().to_string())
            }  
            else {
                DateTime::<Utc>::default().to_rfc3339()
            };

            let cover = if auto_collection.unwrap() {
                if let Some(itch_url) = get_itch_url_or_none(platforms.clone()) {
                    let res = collect_cover_image(id.as_str(), itch_url).await.unwrap();
                    to_download.push(res.clone());
                    Some(res.1.replace("static/", "")) 
                } else {
                    None
                }
            } else {
                if let PageProperty::Files { files, .. } = properties.get("Cover").unwrap()
                {
                    if let Some(it) = files.get(0) {
                        let file_info = parse_url_to_file_info(&parse_file_url(&it.file)).unwrap();
                        let pos = format!(
                            "static/assets/works/{}/cover.{}",
                            page.id.to_string(),
                            file_info.file_ext,
                        );
                        to_download.push((file_info.cleaned_url.clone(), pos.clone()));
                        Some(pos.replace("static/", ""))
                    } else {
                        None
                    }
                } else {
                    None
                }
            };

            let screenshots: Vec<String> = if auto_collection.unwrap() {
                if let Some(itch_url) = get_itch_url_or_none(platforms.clone()) {
                    let mut res = collect_screenshot_images(id.as_str(), itch_url).await.unwrap();
                    let vec = res.iter().map(|item| {item.1.replace("static/", "")}).collect();
                    to_download.append(&mut res);
                    vec
                } else {
                    vec![]
                }
            } else {
                if let PageProperty::Files { files, .. } = properties.get("Screenshot").unwrap() {
                    let mut vec = Vec::<String>::new();
                    for i in 0..files.len() {
                        let it = files[i].clone();
                        let file_info = parse_url_to_file_info(&parse_file_url(&it.file)).unwrap();
                        let pos = format!(
                            "static/assets/works/{}/screenshot_{}.{}",
                            page.id.to_string(),
                            i,
                            file_info.file_ext
                        );
                        to_download.push((file_info.cleaned_url.clone(), pos.clone()));
                        vec.push(pos.replace("static/", ""));
                    }
                    vec
                } else {
                    vec![]
                }
            };
            
            let class = if let PageProperty::Select { id, select } = properties.get("Class").unwrap() {
                if select.name.clone().unwrap_or_default() == "Spotlight" {
                    Class::Spotlight
                } else {
                    Class::Normal
                } 
            } else { Class::Normal };
            Work {
                id,
                name,
                sub_name,
                introduce,
                tags,
                auto_collection,
                platforms,
                authors,
                submission_date: DateTimeUtc {
                    date_rfc3339: submission_date,
                },
                gamejams,
                nova_gamejams,
                cover,
                screenshots,
                last_edited_date: DateTimeUtc {
                    date_rfc3339: page.last_edited_time.to_rfc3339(),
                },
                class,
            }
        });
    }

    // Download files
    for target in to_download {
        download(target.0, target.1).await?;
    }

    // Write json files
    for work in works_objects {
        tool::serialize_to_json_file(&work, format!("data/{}{}.json", path, work.id))?;
    }
    Ok(())
}

fn get_itch_url_or_none(platforms: Vec::<Platform>) -> Option<String> {
    for element in platforms {
        if element.platform_type == PlatformType::Itch {
            return Some(element.url)
        }
    }
    None
}

fn parse_authors(author: &PageProperty, author_link: &PageProperty) -> Vec<Author> {
    let authors_string = get_plain_text_or_none(author).unwrap_or_default();
    let authors_links_string = get_plain_text_or_none(author_link).unwrap_or_default();
    let au_silices = authors_string.split(',').collect::<Vec<_>>();
    let binding = authors_links_string.replace(" ", "");
    let al_silices = binding.split(',').collect::<Vec<_>>();

    let mut authors = Vec::<Author>::new();
    if au_silices.len() == al_silices.len() {
        for i in 0..au_silices.len() {
            let link = al_silices[i];
            let url = if !link.is_empty() {
                Some(link.to_string())
            } else {
                None
            };
            authors.push(Author {
                name: au_silices[i].to_string(),
                url,
            })
        }
    } else {
        let link = al_silices[0];
        let url = if !link.is_empty() {
            Some(link.to_string())
        } else {
            None
        };
        for name in au_silices {
            authors.push(Author {
                name: name.to_string(),
                url: url.clone(),
            })
        }
    }
    authors
}

fn push_platform_when_exist(
    props: &HashMap<String, PageProperty>,
    name: &str,
    vec: &mut Vec<Platform>,
    platform_type: PlatformType
) {
    if let Some(PageProperty::Url { id, url }) = props.get(name) {
        if let Some(it) = url {
            vec.push(Platform {
                platform_type,
                url: it.clone(),
            });
        }
    }
}

pub fn get_multi_selected_or_none(prop: &PageProperty) -> Vec<SelectedValue> {
    if let PageProperty::MultiSelect { id, multi_select } = prop {
        multi_select
            .clone()
            .iter()
            .map(into_selected_value)
            .collect()
    } else {
        vec![]
    }
}

pub fn get_plain_text_or_none(prop: &PageProperty) -> Option<String> {
    if let PageProperty::RichText { id, rich_text } = prop {
        Some(tool::get_plain_string(rich_text))
    } else {
        None
    }
}

pub fn get_checkbox_or_none(prop: &PageProperty) -> Option<bool> {
    if let PageProperty::Checkbox { id: _, checkbox } = prop {
        Some(checkbox.clone())
    } else {
        None
    }
}

pub async fn collect_cover_image(id: &str, itch_url: String) -> Result<(String, String)> {
    let mut result: (String, String) = (String::new(), String::new());
    
    print!("Collect cover for \"{}\": ", id);
    
    // Get Response
    let response = reqwest::get(itch_url).await;
    match  response {
        Err(e) => println!("Failed when connecting server! {}", e),
        _ => {
            // Collect element
            let document = scraper::Html::parse_document(&response.unwrap().text().await?);
            let div_selector = scraper::Selector::parse("div.header").unwrap();
            let img_selector = scraper::Selector::parse("img").unwrap();
            let div = document.select(&div_selector).next().unwrap();
            let list = div.select(&img_selector);
            println!("Success.");

            // Get link and download
            let src = list.last().unwrap().value().attr("src").unwrap().to_string();
            let file_info = parse_url_to_file_info(&src).unwrap();
            let path = format!(
                "static/assets/works/{}/cover.{}",
                id,
                file_info.file_ext
            );
            result = (src, path);
        },
    }
    Ok(result)
}

pub async fn collect_screenshot_images(id: &str, itch_url: String) -> Result<Vec<(String, String)>> {
    let mut result : Vec<(String, String)> = Vec::new();

    print!("Collect screenshots for \"{}\": ", id);

    // Get Response
    let response = reqwest::get(itch_url).await;
    match  response {
        Err(e) => println!("Failed when connecting server! {}", e),
        _ => {
            // Collect elements
            let document = scraper::Html::parse_document(&response.unwrap().text().await?);
            let div_selector = scraper::Selector::parse("div.screenshot_list").unwrap();
            let a_selector = scraper::Selector::parse("a").unwrap();
            let div = document.select(&div_selector).next().unwrap();
            let list = div.select(&a_selector);
            println!("Find {} screenshots.", list.clone().count());

            // Get links and download
            let mut index = 0;
            for element in list {
                let href = element.value().attr("href").unwrap().to_string();
                let file_info = parse_url_to_file_info(&href).unwrap();
                let path = format!(
                    "static/assets/works/{}/screenshot_{}.{}",
                    id,
                    index,
                    file_info.file_ext
                );
                result.push((href, path));
                index += 1;
            }
        },
    }
    Ok(result)
}

pub async fn download(url: String, path: String) -> Result<()> {
    print!("Download \"{}\" to \"{}\": ", url.clone(), path.clone());

    let response = reqwest::get(url.clone()).await;
    match response {
        Err(e) => println!("Failed when connecting server! {}", e),
        _ => {
            // Create folders if not exist
            let pos = path.clone().rfind("/").unwrap_or_default();
            let dir = path.clone().split_at(pos).0.to_string();
            fs::create_dir_all(dir).unwrap();

            // Create file
            let file = File::create(path);
            match file {
                Err(e) => println!("Failed when creating folder! {}", e),
                _ => {
                    // Write data
                    let result = file.unwrap().write_all(&response.unwrap().bytes().await?);
                    match result {
                        Err(e) => println!("Failed when writing files! {}", e),
                        _ => println!("Success."),
                    }
                }
            }
        }
    }
    Ok(())
}

fn into_selected_value(it: &SelectPropertyValue) -> SelectedValue {
    SelectedValue {
        name: it.name.clone().unwrap_or_default(),
    }
}
//return read works sorted by last_edited_date
pub fn read_works(folder_path: &str) -> Result<Vec<Work>> {
    //serde_json::from_str
    let read = fs::read_dir(folder_path)?;
    let mut ret = Vec::<Work>::new();
    for read_dir in read {
        let unwrap = read_dir?;
        let file_type = unwrap.file_type()?;
        let path = unwrap.path();
        let path_ext = path.extension().unwrap_or(OsStr::new(""));
        let is_json_file = file_type.is_file() && path_ext == "json";
        if is_json_file {
            let string = fs::read_to_string(path)?;
            let work = serde_json::from_str::<Work>(string.as_str())?;
            ret.push(work);
        }
    }
    ret.sort_by(|a, b| {
        let a = DateTime::parse_from_rfc3339(a.last_edited_date.date_rfc3339.as_str()).unwrap();
        let b = DateTime::parse_from_rfc3339(b.last_edited_date.date_rfc3339.as_str()).unwrap();

        a.cmp(&b)
    });
    Ok(ret)
}

#[cfg(test)]
mod test {

}
