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
    let works_objects: Vec<Work> = works_pages
        .iter()
        .map(|page| {
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
            let gamejams = get_multi_selected_or_none(&properties.get("GameJam").unwrap());
            let nova_gamejams = get_multi_selected_or_none(&properties.get("NovaGameJam").unwrap());
            let mut platforms = Vec::<Platform>::new();
            push_platform_when_exist(&properties, "Itch", &mut platforms);
            push_platform_when_exist(&properties, "Steam", &mut platforms);
            push_platform_when_exist(&properties, "GameCore", &mut platforms);
            push_platform_when_exist(&properties, "HomePage", &mut platforms);

            let authors = parse_authors(
                &properties.get("Author").unwrap(),
                &properties.get("AuthorLink").unwrap(),
            );
            let submission_date = if let PageProperty::Date {
                date:
                    Some(DatePropertyValue {
                        start: Some(DateOrDateTime::DateTime(it)),
                        ..
                    }),
                ..
            } = properties.get("SubmissionDate").unwrap()
            {
                it.clone()
            } else {
                DateTime::<Utc>::default()
            };

            let cover = if let PageProperty::Files { files, .. } = properties.get("Cover").unwrap()
            {
                if let Some(it) = files.get(0) {
                    let file_info = parse_url_to_file_info(&parse_file_url(&it.file)).unwrap();
                    let pos = format!(
                        "static/assets/works/{}/cover.{}",
                        page.id.to_string(),
                        file_info.file_ext,
                    );
                    to_download.push((file_info.url.clone(), pos.clone()));
                    Some(pos.replace("static/", ""))
                } else {
                    None
                }
            } else {
                None
            };

            let screenshots: Vec<String> =
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
                        to_download.push((file_info.url.clone(), pos.clone()));
                        vec.push(pos.replace("static/", ""));
                    }
                    vec
                } else {
                    vec![]
                };

            Work {
                id,
                name,
                sub_name,
                introduce,
                tags,
                platforms,
                authors,
                submission_date: DateTimeUtc {
                    date_rfc3339: submission_date.to_rfc3339(),
                },
                gamejams,
                nova_gamejams,
                cover,
                screenshots,
                last_edited_date: DateTimeUtc {
                    date_rfc3339: page.last_edited_time.to_rfc3339(),
                },
            }
        })
        .collect();

    // Download files
    for target in to_download {
        let url = target.0;
        let response = reqwest::get(url.clone()).await?;
        let download_path = target.1;

        let file_info = parse_url_to_file_info(&download_path).unwrap();

        println!(
            "downloading file: '{}' to '{}'",
            file_info.cleaned_url, download_path
        );

        fs::create_dir_all(file_info.path.clone())?;
        let mut file = File::create(download_path.to_string())?;
        file.write(&response.bytes().await?)?;

        println!(
            "download successfully! filename: '{}'",
            file_info.file_name_with_ext()
        );
    }

    // Write json files
    for work in works_objects {
        tool::serialize_to_json_file(&work, format!("data/{}{}.json", path, work.id))?;
    }
    Ok(())
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
) {
    if let Some(PageProperty::Url { id, url }) = props.get(name) {
        if let Some(it) = url {
            vec.push(Platform {
                platform_type: PlatformType::Itch,
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

pub async fn collect_cover_image(id: &str, itch_url: String) -> Result<String> {
    // collect to /assets/works/{id}/cover.xxx
    todo!()
}

pub async fn collect_screenshot_images(id: &str, itch_url: String) -> Result<Vec<String>> {
    // collect to /assets/works/{id}/screenshots/screenshot_0.xxx
    todo!()
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

    #[test]
    fn test_read() {
        let a = super::read_works("../../data/works").unwrap();
        assert_eq!(a.len(), 1usize);
    }
}
