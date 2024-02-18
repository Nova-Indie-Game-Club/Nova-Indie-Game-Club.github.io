use rusticnotion::{
    chrono::NaiveDate,
    models::{properties::SelectedValue, DateTime, Utc},
};
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
    pub tags: Vec<SelectedValue>,
    pub gamejams: Vec<SelectedValue>,
    pub nova_gamejams: Vec<SelectedValue>,
    pub platforms: Vec<Platform>,
    pub authors: Vec<Author>,
    pub submission_date: DateTime<Utc>,
    /// assets path of cover.
    pub cover: Option<String>,
    /// assets pathes of screenshots.
    pub screenshots: Vec<String>,
}
