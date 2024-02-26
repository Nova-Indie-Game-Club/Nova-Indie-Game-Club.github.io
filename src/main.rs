mod templates;
mod components;

use perseus::prelude::*;

#[perseus::main(perseus_axum::dflt_server)]
pub fn perseus<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(templates::index::get_template())
        .template(templates::about::get_template())
}

pub fn statics(path: &str) -> String{
    format!(".perseus/static/{}", path)
}