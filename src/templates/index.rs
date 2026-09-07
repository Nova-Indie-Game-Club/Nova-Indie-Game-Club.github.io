use maud::{html, Markup};
use website_model::work::{Class, Work};

use super::content::ABOUT_SECTIONS;
use crate::{
    components::{footer, work},
    tool::statics,
};

fn section(name: &str, title_path: &str, title: &str, body: Markup) -> Markup {
    html! {
        section class=(format!("section {name}")) aria-label=(title) {
            img.title src=(statics(title_path)) alt=(title);
            div.core { (body) }
        }
    }
}

pub fn render(works: &[Work]) -> Markup {
    let recent: Vec<_> = works.iter().take(9).collect();
    let recommended: Vec<_> = works
        .iter()
        .filter(|work| work.class == Class::Spotlight)
        .collect();
    super::page(
        "Nova 独游社!",
        "",
        html! {
            (work::dialog(&recent))
            header {
                nav.navi aria-label="主导航" {
                    div.left { img src=(statics("assets/images/logo_small.png")) alt="Nova Logo"; }
                    div.right {
                        a.option.selected href="./" aria-current="page" { p { "主页" } }
                    }
                }
            }
            div.cover {
                div.logo_title {
                    img.logo src=(statics("assets/images/logo_normal.png")) alt="Nova 独游社";
                    img.title src=(statics("assets/images/cover_title.png")) alt="Nova Indie Game Club";
                }
            }
            main.main {
                (section("about", "assets/images/title_about.png", "关于我们", html! {
                    div.options role="tablist" aria-label="关于我们" {
                        @for (index, (id, title, _)) in ABOUT_SECTIONS.iter().enumerate() {
                            button class=(if index == 0 { "option selected" } else { "option" })
                                id=(format!("about-tab-{id}")) type="button" role="tab"
                                data-about-tab=(id) aria-controls=(format!("about-panel-{id}"))
                                aria-selected=(if index == 0 { "true" } else { "false" })
                                tabindex=(if index == 0 { "0" } else { "-1" }) {
                                span { (title) }
                            }
                        }
                    }
                    div.text-container {
                        @for (index, (id, _, text)) in ABOUT_SECTIONS.iter().enumerate() {
                            p class=(if index == 0 { "text active" } else { "text" })
                                id=(format!("about-panel-{id}")) role="tabpanel" data-about-panel=(id)
                                aria-labelledby=(format!("about-tab-{id}")) hidden[index != 0] { (text) }
                        }
                    }
                }))
                (section("recommended-works", "assets/images/title_recommended_works.png", "推荐作品", work::recommended(&recommended)))
                (section("recent-works", "assets/images/title_recent_works.png", "近期作品", html! {
                    div.works-container {
                        @for item in &recent { (work::recent_card(item)) }
                    }
                }))
                (section("media", "assets/images/title_media.png", "媒体", html! {
                    a.item.theme-shadow-decoration href="https://mp.weixin.qq.com/s/vgXY1kWc0qa-n7RPXlSmwA"
                        target="_blank" rel="noopener noreferrer" {
                        img src=(statics("assets/images/qr_code/wechat.jpeg")) alt="微信公众号二维码" loading="lazy";
                        div { "微信公众号" }
                    }
                    a.item.theme-shadow-decoration href="https://space.bilibili.com/406914779"
                        target="_blank" rel="noopener noreferrer" {
                        img src=(statics("assets/images/qr_code/bilibili.png")) alt="Bilibili 二维码" loading="lazy";
                        div { "Bilibili" }
                    }
                }))
                noscript { p { "启用 JavaScript 后可切换作品、查看作品详情和截图。" } }
            }
            (footer::render())
        },
    )
}
