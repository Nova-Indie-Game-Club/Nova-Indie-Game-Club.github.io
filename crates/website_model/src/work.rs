
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
}

impl Work {
    pub fn plain_submission_date_day(&self) -> String{
        self.submission_date.date_rfc3339.split_once("T").unwrap().0.to_string()
    }
    pub fn plain_author_string(&self) -> String{
        self.authors.iter().map(|it| {it.name.clone()}).collect::<Vec<_>>().join(", ")
    }
}
