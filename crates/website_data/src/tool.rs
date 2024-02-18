
use anyhow::*;
use notion_client::objects::{file, rich_text::RichText};
use serde::{Deserialize, Serialize};

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
    pub fn file_name_with_ext(&self) -> String{
        format!("{}.{}", self.file_name, self.file_ext)
    }
}
pub fn parse_file_url(it: &file::File) -> String {
    let url = match it {
        file::File::External { external } => external.url.clone(),
        file::File::File { file } => file.url.clone(),
    };
    url
}

pub fn parse_url_to_file_info(raw_url: &String) -> Result<FileInfo> {
    let mut url = raw_url.clone();
    if let Some(it) = url.rfind("?"){
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
        path: split0.0.to_string()
    })
}

#[cfg(test)]
mod test {
    use super::parse_url_to_file_info;

    #[test]
    fn test_parse_url() {
        let info = parse_url_to_file_info(
            &"https://github.com/tokio-rs/tokio/workflows/CI/badge.svg?3123124scxsdge32f".to_string(),
        )
        .unwrap();
        assert_eq!(
            info.url,
            "https://github.com/tokio-rs/tokio/workflows/CI/badge.svg?3123124scxsdge32f"
        );
        assert_eq!(info.cleaned_url,
            "https://github.com/tokio-rs/tokio/workflows/CI/badge.svg"
        );
        assert_eq!(info.file_name, "badge");
        assert_eq!(info.file_ext, "svg");
        assert_eq!(info.path, "https://github.com/tokio-rs/tokio/workflows/CI")
    }
}

