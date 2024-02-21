use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use website_data::model::work::Work;

use crate::statics;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "IndexPageStateRx")]
struct IndexPageState {
    recent_works: Vec<Work>,
}

#[auto_scope]
fn index_page<G: Html>(cx: Scope, state: &IndexPageStateRx) -> View<G> {
    view! { cx,
        header{
            div(class="navi"){
                div(class="left"){
                    img(src=(statics("assets/images/logo_small.png")), alt="Nova Logo")
                }
                div(class="right"){
                    div(class="option selected"){
                        p{ "主页" }
                    }
                    div(class="option"){
                        p{ "作品" }
                    }
                }
            }
        }
        div(class="cover"){
            img(src=(statics("assets/images/cover_background.png")), alt="Background", class="background")
            div(class="logo_title"){
                img(src=(statics("assets/images/logo_normal.png")), alt="logo", class="Logo")
                img(src=(statics("assets/images/cover_title.png")), alt="title", class="Title")
            }
        }
        Section(name="recent-works".to_string(),title_path="assets/images/title_recent_works.png".to_string()){
            div(class="works-container"){
                Indexed(
                    iterable=&state.recent_works,
                    view=|cx, it| {
                        let cover = it.cover.clone().unwrap();
                        view!{ cx,
                            RecentWorkItem(
                                cover_path=cover,
                                date=it.plain_submission_date_day(),
                                author=it.plain_author_string(),
                                title=it.name
                            )
                        }
                    },
                )
            }
        }
    }
}
#[derive(Prop)]
pub struct SectionProps<'a, G: Html> {
    pub name: String,
    pub title_path: String,
    pub children: Children<'a, G>,
}
#[component]
fn Section<'a, G: Html>(cx: Scope<'a>, props: SectionProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);
    let name_clone = props.name.clone();
    view!(cx,
        div(class=(format!("section {}", props.name))){ //todo 检查
            img(src=(statics(props.title_path.as_str())), alt=(name_clone))
            div(class="core"){
                (children)
            }
        }
    )
}
#[derive(Prop)]
pub struct RecentWorkItemProps{
    pub cover_path: String,
    pub date: String,
    pub author: String,
    pub title: String
}
#[component]
fn RecentWorkItem<G: Html>(cx: Scope, props: RecentWorkItemProps) -> View<G> {
    view!(cx,
        div(class="item"){
            div(class="up-part"){
                img(src=(statics(props.cover_path.as_str())), alt="Cover")
                div(class="info"){
                    p{(props.date)}
                    p{(format!("by {}", props.author))}
                }
            }
            div(class="title-text"){
                p{ (props.title) }
            }
        }
    )
}

#[engine_only_fn]
fn head(cx: Scope, _props: IndexPageState) -> View<SsrNode> {
    view! { cx,
        title { "Nova 独游社!" }
        link(rel="stylesheet",herf=(statics("css/index.css")))
    }
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> IndexPageState {
    let works = website_data::read_works().unwrap();

    //Get first recent 9 works;
    let recent_works = works.iter().take(9).cloned().collect::<Vec<_>>();
    IndexPageState { recent_works }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .head_with_state(head)
        .build()
}
