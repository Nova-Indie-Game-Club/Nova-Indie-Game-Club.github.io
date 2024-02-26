
use crate::components::work_item::*;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use web_sys::Element;
use website_model::work::Work;

use crate::statics;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "IndexPageStateRx")]
struct IndexPageState {
    recent_works: Vec<Work>,
}

pub static ABOUT_INTRODUCE_TEXT: &str = "Nova独游社全称Nova独立游戏社，成立于2018年冬，前身是西电软院科协独立游戏组，本部位于西安电子科技大学，在西安邮电大学、陕西科技大学设有分部。传承和发展独立游戏文化及独立游戏的独立精神成为西安地区高校学生游戏开发者聚集地成为西安地区高校游戏爱好者的聚集地。";
pub static ABOUT_ACTIVITIES_TEXT: &str = "Nova独游社全称Nova独立游戏社，成立于2018年冬，前身是西电软院科协独立游戏组，本部位于西安电子科技大学，在西安邮电大学、陕西科技大学设有分部。传承和发展独立游戏文化及独立游戏的独立精神成为西安地区高校学生游戏开发者聚集地成为西安地区高校游戏爱好者的聚集地。";
pub static ABOUT_JOIN_US_TEXT: &str = "Nova独游社全称Nova独立游戏社，成立于2018年冬，前身是西电软院科协独立游戏组，本部位于西安电子科技大学，在西安邮电大学、陕西科技大学设有分部。传承和发展独立游戏文化及独立游戏的独立精神成为西安地区高校学生游戏开发者聚集地成为西安地区高校游戏爱好者的聚集地。";

pub static ID_ABOUT_INTRODUCE: &str = "about_introduce";
pub static ID_ABOUT_ACTIVITIES: &str = "about_activities";
pub static ID_ABOUT_JOIN_US: &str = "about_join_us";
pub static ID_ABOUT_INTRODUCE_TEXT: &str = "about_introduce_text";
pub static ID_ABOUT_ACTIVITIES_TEXT: &str = "about_activities_text";
pub static ID_ABOUT_JOIN_US_TEXT: &str = "about_join_us_text";

fn id_selector(input: &str) -> String{
    format!("#{}", input)
}

fn on_about_section_option_clicked(switch_to: AboutType, text: &Signal<&str>) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let introduce = document
        .query_selector(id_selector(ID_ABOUT_INTRODUCE).as_str())
        .unwrap()
        .unwrap();
    let activities = document
        .query_selector(id_selector(ID_ABOUT_ACTIVITIES).as_str())
        .unwrap()
        .unwrap();
    let join_us = document.query_selector(id_selector(ID_ABOUT_JOIN_US).as_str()).unwrap().unwrap();

    let introduce_text = document.query_selector(id_selector(ID_ABOUT_INTRODUCE_TEXT).as_str()).unwrap().unwrap();
    let activities_text = document.query_selector(id_selector(ID_ABOUT_ACTIVITIES_TEXT).as_str()).unwrap().unwrap();
    let join_us_text = document.query_selector(id_selector(ID_ABOUT_JOIN_US_TEXT).as_str()).unwrap().unwrap();

    for it in [&introduce, &activities, &join_us] {
        if it.class_list().contains("selected") {
            it.class_list().remove_1("selected").unwrap();
        }
    }
    for it in [&introduce_text, &activities_text, &join_us_text] {
        if it.class_list().contains("active"){
            it.class_list().remove_1("active").unwrap()
        }
    }

    fn do_select(content: &'static str, ele: &Element, text: &Signal<&str>, text_ele: &Element) {
        text.set(content);
        ele.class_list().add_1("selected").unwrap();
        text_ele.class_list().add_1("active").unwrap();
    }
    match switch_to {
        AboutType::Introduce => do_select(ABOUT_INTRODUCE_TEXT, &introduce, text, &introduce_text),
        AboutType::Activities => do_select(ABOUT_ACTIVITIES_TEXT, &activities, text, &activities_text),
        AboutType::JoinUs => do_select(ABOUT_JOIN_US_TEXT, &join_us, text, &join_us_text),
    }
}
enum AboutType {
    Introduce,
    Activities,
    JoinUs,
}
#[auto_scope]
fn index_page<G: Html>(cx: Scope, state: &IndexPageStateRx) -> View<G> {
    let about_section_text = create_signal(cx, ABOUT_INTRODUCE_TEXT);
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
                img(src=(statics("assets/images/logo_normal.png")), alt="Logo", class="logo")
                img(src=(statics("assets/images/cover_title.png")), alt="Title", class="title")
            }
        }
        div(class="main"){
            Section(name="about".to_string(),title_path="assets/images/title_about.png".to_string()){
                div(class="options"){
                    div(class="option selected", id="about_introduce", on:click=|_| on_about_section_option_clicked(AboutType::Introduce, about_section_text)){
                        p{"介绍"}
                    }
                    div(class="option", id="about_activities", on:click=|_| on_about_section_option_clicked(AboutType::Activities, about_section_text)){
                        p{"活动"}
                    }
                    div(class="option", id="about_join_us", on:click=|_| on_about_section_option_clicked(AboutType::JoinUs, about_section_text)){
                        p{"招新"}
                    }
                }
                div(class="text-container"){
                    p(class="text active", id="about_introduce_text"){
                        (ABOUT_INTRODUCE_TEXT)
                    }
                    p(class="text", id="about_activities_text"){
                        (ABOUT_ACTIVITIES_TEXT)
                    }
                    p(class="text", id="about_join_us_text"){
                        (ABOUT_JOIN_US_TEXT)
                    }
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
            img(src=(statics(props.title_path.as_str())), alt=(name_clone), class="title")
            div(class="core"){
                (children)
            }
        }
    )
}

#[engine_only_fn]
fn head(cx: Scope, _props: IndexPageState) -> View<SsrNode> {
    view! { cx,
        title { "Nova 独游社!" }
        link(rel="stylesheet",href=(statics("css/index.css")))
    }
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> IndexPageState {
    let works = website_data::read_works("data/works").unwrap();
    println!("{} works read!", works.len());

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
