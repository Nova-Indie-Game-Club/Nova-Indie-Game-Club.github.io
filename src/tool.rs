pub fn statics(path: &str) -> String {
    format!("./.perseus/static/{}", path)
}

pub fn link_in_new_tab(link: &str) -> String {
    format!("window.open('{}')", link)
}
