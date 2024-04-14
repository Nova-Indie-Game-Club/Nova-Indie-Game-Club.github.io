use anyhow::*;
use notion_client::objects::{file, rich_text::RichText};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
};

pub fn get_plain_string(vec: &Vec<RichText>) -> String {
    vec.iter()
        .map(|it| {
            if let RichText::Text {
                plain_text: Some(plain_text),
                ..
            } = it
            {
                plain_text.clone()
            } else {
                String::default()
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub url: String,
    pub cleaned_url: String,
    //without file extensiton
    pub file_name: String,
    //without '.'
    pub file_ext: String,
    pub path: String,
}

impl FileInfo {
    pub fn file_name_with_ext(&self) -> String {
        format!("{}.{}", self.file_name, self.file_ext)
    }
}
pub fn parse_file_download_url(it: &file::File, id: &String) -> String {
    let url = match it {
        file::File::External { external } => external.url.clone(),
        file::File::File { file } => {
            let url = file.url.clone();
            url
        }
    };
    url
}

pub fn parse_file_info(it: &file::File) -> Result<FileInfo> {
    let url = match it {
        file::File::External { external } => external.url.clone(),
        file::File::File { file } => file.url.clone(),
    };
    parse_url_to_file_info(&url)
}

pub fn parse_url_to_file_info(raw_url: &String) -> Result<FileInfo> {
    let mut url = raw_url.clone();
    if let Some(it) = url.rfind("?") {
        url = url.split_at(it).0.to_string();
    }

    let pos0 = url.rfind("/").expect("url format is wrong!");
    let split0 = url.split_at(pos0);
    let mut split = split0.1.to_string();
    split.remove(0);
    let pos1 = split.rfind(".").expect("url format is wrong!");
    let split = split.split_at(pos1);

    Ok(FileInfo {
        file_name: split.0.to_string(),
        file_ext: {
            let mut ret = split.1.to_string();
            ret.remove(0);
            ret
        },
        url: raw_url.clone(),
        cleaned_url: url.clone(),
        path: split0.0.to_string(),
    })
}

pub fn serialize_to_json_file<T>(obj: &T, path: String) -> Result<()>
where
    T: ?Sized + Serialize,
{
    let json = serde_json::to_string(obj)?;
    let info = parse_url_to_file_info(&path)?;
    fs::create_dir_all(info.path.clone())?;
    File::create(path)?.write(json.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::parse_url_to_file_info;

    #[test]
    fn test_parse_url() {
        let info = parse_url_to_file_info(
            &"https://github.com/tokio-rs/tokio/workflows/CI/badge.svg?3123124scxsdge32f"
                .to_string(),
        )
        .unwrap();
        assert_eq!(
            info.url,
            "https://github.com/tokio-rs/tokio/workflows/CI/badge.svg?3123124scxsdge32f"
        );
        assert_eq!(
            info.cleaned_url,
            "https://github.com/tokio-rs/tokio/workflows/CI/badge.svg"
        );
        assert_eq!(info.file_name, "badge");
        assert_eq!(info.file_ext, "svg");
        assert_eq!(info.path, "https://github.com/tokio-rs/tokio/workflows/CI")
    }
}
