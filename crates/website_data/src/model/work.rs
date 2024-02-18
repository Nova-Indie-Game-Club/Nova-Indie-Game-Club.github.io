
use chrono::{DateTime, Utc};
use notion_client::objects::{page, property::Color};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Author {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PlatformType {
    Itch,
    Steam,
    GameCore,
    Homepage,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Platform {
    pub platform_type: PlatformType,
    pub url: String,
}



#[derive(Serialize, Deserialize, Clone)]
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
}
