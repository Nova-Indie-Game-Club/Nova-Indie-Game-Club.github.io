use nova_website::templates;
use nova_website::error_views;
use perseus::prelude::*;

#[perseus::main_export]
pub fn perseus<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(templates::index::get_template())
        .template(templates::about::get_template())
        .error_views(error_views::get_error_views())
}
