
use std::str;

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
    HomePage,
}
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Platform {
    pub platform_type: PlatformType,
    pub url: String,
}
impl PlatformType {
    pub fn display_name(&self) -> String{
        match self {
            PlatformType::Itch => "itch.io".to_string(),
            PlatformType::Steam => "Steam".to_string(),
            PlatformType::GameCore => "机核".to_string(),
            PlatformType::HomePage => "主页".to_string(),
        }
    }

    pub fn type_name(&self) -> String{
        match self {
            PlatformType::Itch => "itch".to_string(),
            PlatformType::Steam => "steam".to_string(),
            PlatformType::GameCore => "gamecore".to_string(),
            PlatformType::HomePage => "homepage".to_string(),
        }
    }

    pub fn from_en_id(id: &str) -> Option<Self>{
        if id == "Itch" {
            return Some(PlatformType::Itch);
        }
        if id == "GameCore" {
            return Some(PlatformType::GameCore);
        }
        if id == "Steam" {
            return Some(PlatformType::Steam);
        }
        if id == "HomePage" {
            return Some(PlatformType::HomePage);
        }
        return Option::None;
    }
}


#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum Class {
    Spotlight,
    Normal
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct SelectedValue{
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct DateTimeUtc{
    pub date_rfc3339: String
}


#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Work {
    pub id: String,
    pub name: String,
    pub sub_name: Option<String>,
    pub introduce: String,
    pub tags:  Vec<SelectedValue>,
    pub gamejams: Vec<SelectedValue>,
    pub nova_gamejams: Vec<SelectedValue>,
    pub platforms: Vec<Platform>,
    pub authors: Vec<Author>,
    /// assets path of cover.
    pub cover: Option<String>,
    /// assets pathes of screenshots.
    pub screenshots: Vec<String>,
    pub submission_date: DateTimeUtc,
    pub last_edited_date: DateTimeUtc,
    pub class: Class,
    pub auto_collection: Option<PlatformType>,
}

impl Work {
    pub fn plain_submission_date_day(&self) -> String{
        self.submission_date.date_rfc3339.split_once("T").unwrap().0.to_string()
    }
    pub fn plain_author_string(&self) -> String{
        self.authors.iter().map(|it| {it.name.clone()}).collect::<Vec<_>>().join(", ")
    }
}
