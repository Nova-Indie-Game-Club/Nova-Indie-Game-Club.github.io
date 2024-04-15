use crate::components::footer::FooterItem;
use crate::components::{footer::Footer, work::*};
use crate::tool::statics;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use web_sys::Element;
use website_model::work::{Class, Work};

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "IndexPageStateRx")]
struct IndexPageState {
    recent_works: Vec<Work>,
    recommended_works: Vec<Work>,
}

pub static ABOUT_INTRODUCE_TEXT: &str = "我们是一个主要在西安电子科技大学活动的大学生独立游戏社团。我们的主要活动内容是制作游戏和欣赏游戏。Nova 独游社的前身是西电软院科协独立游戏组，现在的 Nova 独游社成立于 2018 年冬。我们会在线上社群和线下聚会中讨论游戏开发相关技术、品鉴游戏设计方法和交流游戏及泛游戏社区内容等。更重要的是，我们会制作游戏、互相试玩彼此的游戏并交流想法和感受。我们欢迎对游戏设计及开发感兴趣，或希望尝试开发自己的游戏的朋友。";
pub static ABOUT_ACTIVITIES_TEXT: &str = "我们会举办定期的线下活动：新生见面会、春/秋季见面会是社团较大型的活动，会集中性地分享游戏设计、开发知识，分享项目开发经验等等多方面的内容；新生 GameJam、春/秋季 GameJam 是社团举办的限时限定主题的游戏开发活动，制作完游戏后，我们会举办路演会供大家互相展示和试玩彼此的作品。此外，我们还会持续性地举办不定期的线下活动：Nova 分享会以某个具体的主题为中心，交流相关的信息和知识。我们还会举办一些线上活动：包括由大家投稿的 Nova 测评（文章）、游戏主题杂谈的 Nova 电台（音频）。";
pub static ABOUT_JOIN_US_TEXT: &str = "我们会在每年的 9~10 月开始招新。招新信息会发布在我们的微信公众号 “Nova 独游社” 中（或见网站下面的“媒体”部分），请有意向的朋友关注。我们欢迎对游戏设计及开发感兴趣，或希望尝试开发自己的游戏的朋友。";

pub static ID_ABOUT_INTRODUCE: &str = "about_introduce";
pub static ID_ABOUT_ACTIVITIES: &str = "about_activities";
pub static ID_ABOUT_JOIN_US: &str = "about_join_us";
pub static ID_ABOUT_INTRODUCE_TEXT: &str = "about_introduce_text";
pub static ID_ABOUT_ACTIVITIES_TEXT: &str = "about_activities_text";
pub static ID_ABOUT_JOIN_US_TEXT: &str = "about_join_us_text";

fn id_selector(input: &str) -> String {
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
    let join_us = document
        .query_selector(id_selector(ID_ABOUT_JOIN_US).as_str())
        .unwrap()
        .unwrap();

    let introduce_text = document
        .query_selector(id_selector(ID_ABOUT_INTRODUCE_TEXT).as_str())
        .unwrap()
        .unwrap();
    let activities_text = document
        .query_selector(id_selector(ID_ABOUT_ACTIVITIES_TEXT).as_str())
        .unwrap()
        .unwrap();
    let join_us_text = document
        .query_selector(id_selector(ID_ABOUT_JOIN_US_TEXT).as_str())
        .unwrap()
        .unwrap();

    for it in [&introduce, &activities, &join_us] {
        if it.class_list().contains("selected") {
            it.class_list().remove_1("selected").unwrap();
        }
    }
    for it in [&introduce_text, &activities_text, &join_us_text] {
        if it.class_list().contains("active") {
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
        AboutType::Activities => {
            do_select(ABOUT_ACTIVITIES_TEXT, &activities, text, &activities_text)
        }
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
    // let recent_work_focused_index = create_signal(cx, 0usize);
    view! { cx,
        div(class="recent-work-focused"){
            div(class="mask"){}
            //todo work spotlight
        }
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
            Section(name="recommended-works".to_string(), title_path="assets/images/title_recommended_works.png".to_string()){
                FocusedWorkPanel(works=state.recommended_works.get())
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
            Section(name="media".to_string(),title_path="assets/images/title_media.png".to_string()){
                div{
                    img(src=statics("assets/images/qr_code/wechat.jpeg"), alt="wechat")
                    a(href="https://mp.weixin.qq.com/s/vgXY1kWc0qa-n7RPXlSmwA",target="_blank"){ "微信公众号" }
                }
                div{
                    img(src=statics("assets/images/qr_code/bilibili.png"), alt="bilibili")
                    a(href="https://space.bilibili.com/406914779",target="_blank"){ "Bilibili" }
                }
            }
        }
        Footer(items=vec![
            FooterItem{
                title: "联系我们".to_string(),
                text: "hi@novaindiegame.club".to_string(),
                link: None,
                info: None,
            },
            FooterItem{
                title: "网站信息".to_string(),
                text: "网站仓库".to_string(),
                link: Some("https://github.com/Nova-Indie-Game-Club/nova_website".to_string()),
                info: Some("网站使用 Perseus 框架开发，代码遵循 MIT 开源协议，资源遵循 CC BY-NC-SA 4.0 协议。".to_string()),
            }
            ]
        )
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
        link(rel="stylesheet",href=(statics("css/root.css")))
        link(rel="stylesheet",href=(statics("css/index.css")))
        link(rel="stylesheet",href=(statics("css/recommended_works.css")))
        link(rel="stylesheet",href=(statics("css/work_spotlight.css")))
        link(rel="icon",type="image/x-icon",href=(statics("assets/images/logo_small.png")))
    }
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> IndexPageState {
    let works = website_data::read_works("data/works").unwrap_or_default();
    println!("{} works read!", works.len());

    //Get first recent 9 works;
    let recent_works = works.iter().take(9).cloned().collect::<Vec<_>>();
    let recommended_works: Vec<_> = works
        .iter()
        .filter(|it| it.class == Class::Spotlight)
        .cloned()
        .collect();

    IndexPageState {
        recent_works,
        recommended_works,
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .head_with_state(head)
        .build()
}
