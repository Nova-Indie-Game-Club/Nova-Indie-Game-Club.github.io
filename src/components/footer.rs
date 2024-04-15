use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use crate::{
    components::SvgCode,
    svg,
    tool::{link_in_new_tab, statics},
};

#[derive(Clone, Serialize, Deserialize)]
pub struct FooterItem {
    pub title: String,
    pub text: String,
    pub link: Option<String>,
    pub info: Option<String>,
}

#[derive(Prop)]
pub struct FooterProps {
    pub items: Vec<FooterItem>,
}

#[component]
pub fn Footer<G: Html>(cx: Scope, props: FooterProps) -> View<G> {
    let item_views = View::new_fragment(
        props
            .items
            .iter()
            .map(|it| {
                let it = it.clone();
                let has_info = it.info.is_some();
                let info = it.info.unwrap_or_default();
                if let Some(link) = it.link {
                    view! { cx,
                        div(class="footer-item has-link footer-item-layout", onclick=(link_in_new_tab(link.as_str()))){
                            div(class="up-part"){
                                div(class="left-part"){
                                    div(class="item-title"){ (it.title) }
                                }
                                div(class="right-part"){
                                    div(class="item-text"){ (it.text) }
                                    SvgCode(class="link-icon", code=svg::LINK_ARROW)
                                }
                            }
                            (if has_info {view!{cx,
                                div(class="down-part"){
                                    p{ (info) }
                                }
                            }} else {view!(cx,)})
                        }
                    }
                }else{
                    view!{ cx,
                        div(class="footer-item footer-item-layout"){
                            div(class="up-part"){
                                div(class="left-part"){
                                    div(class="item-title"){ (it.title) }
                                }
                                div(class="right-part"){
                                    div(class="item-text"){ (it.text) }
                                }
                            }
                            (if has_info {view!{cx,
                                div(class="down-part"){
                                    p{ (info) }
                                }
                            }} else {view!(cx,)})
                        }
                    }
                }
            })
            .collect(),
    );
    view! { cx,
        footer{
            div(class="footer-left-part"){
                (item_views)
                div(class="license footer-item-layout", dangerously_set_inner_html="<p xmlns:cc='http://creativecommons.org/ns#' xmlns:dct='http://purl.org/dc/terms/'><span property='dct:title'>Nova Website's assets files</span> by <span property='cc:attributionName'>Nova Indie Game Club</span> is licensed under <a href='http://creativecommons.org/licenses/by-nc-sa/4.0/?ref=chooser-v1' target='_blank' rel='license noopener noreferrer' style='display:inline-block;'>CC BY-NC-SA 4.0<img style='height:22px!important;margin-left:3px;vertical-align:text-bottom;' src='https://mirrors.creativecommons.org/presskit/icons/cc.svg?ref=chooser-v1'><img style='height:22px!important;margin-left:3px;vertical-align:text-bottom;' src='https://mirrors.creativecommons.org/presskit/icons/by.svg?ref=chooser-v1'><img style='height:22px!important;margin-left:3px;vertical-align:text-bottom;' src='https://mirrors.creativecommons.org/presskit/icons/nc.svg?ref=chooser-v1'><img style='height:22px!important;margin-left:3px;vertical-align:text-bottom;' src='https://mirrors.creativecommons.org/presskit/icons/sa.svg?ref=chooser-v1'></a></p>")
            }
        }
    }
}
