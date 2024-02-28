use nova_website::templates;
use perseus::prelude::*;

#[perseus::main(perseus_axum::dflt_server)]
pub fn perseus<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(templates::index::get_template())
        .template(templates::about::get_template())
}


