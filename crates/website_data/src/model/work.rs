
use chrono::{DateTime, Utc};
use notion_client::objects::page;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Author {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum PlatformType {
    Itch,
    Steam,
    GameCore,
    Homepage,
}
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Platform {
    pub platform_type: PlatformType,
    pub url: String,
}



#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Work {
    pub id: String,
    pub name: String,
    pub sub_name: Option<String>,
    pub introduce: String,
    pub tags:  Vec<page::SelectPropertyValue>,
    pub gamejams: Vec<page::SelectPropertyValue>,
    pub nova_gamejams: Vec<page::SelectPropertyValue>,
    pub platforms: Vec<Platform>, 
    pub authors: Vec<Author>,
    pub submission_date: DateTime<Utc>,
    /// assets path of cover.
    pub cover: Option<String>,
    /// assets pathes of screenshots.
    pub screenshots: Vec<String>,
    pub last_edited_date: DateTime<Utc>,
}

impl Work {
    pub fn plain_submission_date_day(&self) -> String{
        self.submission_date.time().format("%Y-%m-%d").to_string()
    }
    pub fn plain_author_string(&self) -> String{
        self.authors.iter().map(|it| {it.name.clone()}).collect::<Vec<_>>().join(", ")
    }
}
