/// URLs are relative to the exported homepage, including template content cloned by JS.
pub fn statics(path: &str) -> String {
    format!("static/{path}")
}
