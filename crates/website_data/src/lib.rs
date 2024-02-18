use anyhow::*;
use model::work::{Author, Platform, PlatformType, Work};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    str::FromStr,
};
use tool::Empty;

use notion::{
    chrono::NaiveWeek,
    ids::{AsIdentifier, DatabaseId},
    models::{
        properties::{DateOrDateTime, DateValue, PropertyValue, SelectedValue},
        search::DatabaseQuery,
        DateTime, Utc,
    },
    NotionApi,
};

pub mod model;
pub mod tool;

const WORKS_DATABASE_ID: &str = "9ff4f3a98e8343acb33defe8f82804bd";

const WORKS_DATA_PATH: &str = "works/";

pub async fn run() -> Result<()> {
    let notion_api = NotionApi::new(std::env::var("NOVA_WEBSITE_NOTION_TOKEN")?)?;

    collect_database_to_file(
        &notion_api,
        DatabaseId::from_str(WORKS_DATABASE_ID).unwrap(),
        WORKS_DATA_PATH.to_string(),
    )
    .await?;
    Ok(())
}

pub async fn collect_database_to_file(
    notion_api: &NotionApi,
    id: DatabaseId,
    path: String,
) -> Result<()> {
    let works_database = notion_api
        .get_database(id)
        .await
        .expect("Getting works database failed!");
    let works_pages = notion_api
        .query_database(works_database.id.clone(), DatabaseQuery::empty())
        .await
        .unwrap()
        .results;

    //(url, native_position)
    let mut to_download: Vec<(String, String)> = vec![];
    let works_objects: Vec<Work> = works_pages
        .iter()
        .map(|page| {
            let properties = &page.properties.properties;
            let id = page.id.as_id().to_string();
            let name = page.properties.title().unwrap();
            let sub_name = get_plain_text_or_none(&properties.get("SubName").unwrap());
            let introduce = get_plain_text_or_none(&properties.get("Introduce").unwrap())
                .unwrap_or("".to_string());
            let tags = get_multi_selected_or_none(&properties.get("Tag").unwrap());
            let gamejams = get_multi_selected_or_none(&properties.get("GameJam").unwrap());
            let nova_gamejams = get_multi_selected_or_none(&properties.get("NovaGameJam").unwrap());
            let mut platforms = Vec::<Platform>::new();
            push_when_exist(&properties, "Itch", &mut platforms);
            push_when_exist(&properties, "Steam", &mut platforms);
            push_when_exist(&properties, "GameCore", &mut platforms);
            push_when_exist(&properties, "HomePage", &mut platforms);

            let authors = parse_authors(
                &properties.get("Author").unwrap(),
                &properties.get("AuthorLink").unwrap(),
            );
            let submission_date = if let PropertyValue::Date {
                date:
                    Some(DateValue {
                        start: DateOrDateTime::DateTime(date),
                        ..
                    }),
                ..
            } = properties.get("SubmissionDate").unwrap()
            {
                date.clone()
            } else {
                DateTime::<Utc>::default()
            };

            let cover = if let PropertyValue::Files {
                files: Some(files), ..
            } = properties.get("Cover").unwrap()
            {
                if let Some(it) = files.get(0) {
                    let pos = format!(
                        "assets/works/{}/cover.{}",
                        page.id.as_id().to_string(),
                        it.mime_type
                    );
                    to_download.push((it.url.to_string(), format!("../{}", pos)));
                    Some(pos)
                } else {
                    None
                }
            } else {
                None
            };

            let screenshots: Vec<String> = if let PropertyValue::Files {
                id,
                files: Some(files),
            } = properties.get("Screenshot").unwrap()
            {
                let mut vec = Vec::<String>::new();
                for i in 0..files.len() {
                    let it = files[i].clone();
                    let pos = format!(
                        "assets/works/{}/screenshot_{}.{}",
                        page.id.as_id().to_string(),
                        i,
                        it.mime_type
                    );
                    to_download.push((it.url.to_string(), format!("../{}", pos)));
                    vec.push(pos);
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
                submission_date,
                gamejams,
                nova_gamejams,
                cover,
                screenshots,
            }
        })
        .collect();

    // Download files
    for target in to_download {
        let url = target.0;
        let response = reqwest::get(url.clone()).await?;
        let download_path = target.1;

        let pos = download_path.rfind('/').unwrap();
        let split = download_path.split_at(pos);

        println!("downloading file: '{}' to '{}'", url, download_path);

        fs::create_dir_all(split.0)?;
        let mut file = File::create(download_path.clone())?;
        file.write(&response.bytes().await?)?;

        println!("download successfully! filename: '{}'", split.1);
    }

    // Write json files
    for work in works_objects {
        let json = serde_json::to_string(&work)?;
        fs::create_dir_all(format!("data/{}", path))?;
        let file_path = format!("data/{}{}.json", path, work.id);
        File::create(file_path)?.write(json.as_bytes())?;
    }
    Ok(())
}

fn parse_authors(author: &PropertyValue, author_link: &PropertyValue) -> Vec<Author> {
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

fn push_when_exist(props: &HashMap<String, PropertyValue>, name: &str, vec: &mut Vec<Platform>) {
    if let Some(PropertyValue::Url { id, url }) = props.get(name) {
        if let Some(it) = url {
            vec.push(Platform {
                platform_type: PlatformType::Itch,
                url: it.clone(),
            });
        }
    }
}
pub fn get_multi_selected_or_none(prop: &PropertyValue) -> Vec<SelectedValue> {
    if let PropertyValue::MultiSelect { id, multi_select } = prop {
        multi_select.clone().unwrap_or(vec![])
    } else {
        vec![]
    }
}

pub fn get_plain_text_or_none(prop: &PropertyValue) -> Option<String> {
    if let PropertyValue::Text { id, rich_text } = prop {
        Some(rich_text.iter().map(|it| it.plain_text()).collect())
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
