use maud::{html, Markup};
use website_model::work::Work;

use crate::tool::statics;

pub fn spotlight(work: &Work) -> Markup {
    let focused = work.screenshots.first().or(work.cover.as_ref());
    html! {
        article.work-spotlight data-work-id=(work.id) {
            h2.work-title { (work.name) }
            @if let Some(image) = focused {
                div.container {
                    div.content {
                        div.game-image.current-image {
                            img data-focused-image src=(statics(image)) alt=(work.name);
                        }
                    }
                }
            }
            @if !work.screenshots.is_empty() {
                div.gallery aria-label="游戏截图" {
                    @for (index, image) in work.screenshots.iter().enumerate() {
                        button.item type="button" data-image-src=(statics(image))
                            aria-label=(format!("查看第 {} 张截图", index + 1))
                            aria-pressed=(if index == 0 { "true" } else { "false" }) {
                            img src=(statics(image)) alt=(format!("{} 截图 {}", work.name, index + 1)) loading="lazy";
                        }
                    }
                }
            }
            div.author { p { "By " (work.plain_author_string()) } }
            div.description { p { (work.introduce) } }
            div.links {
                @for platform in &work.platforms {
                    a.underline-clickable href=(platform.url) target="_blank" rel="noopener noreferrer" {
                        (platform.platform_type.display_name())
                        (super::link_icon())
                    }
                }
            }
        }
    }
}

pub fn recent_card(work: &Work) -> Markup {
    html! {
        article.item.theme-shadow-decoration {
            button.work-card type="button" data-open-work=(work.id) aria-haspopup="dialog" {
                span.up-part {
                    @if let Some(cover) = &work.cover {
                        img src=(statics(cover)) alt=(work.name) loading="lazy";
                    }
                    span.info {
                        span { (work.plain_submission_date_day()) }
                        span { "by " (work.plain_author_string()) }
                    }
                }
                span.title-text { span { (work.name) } }
            }
        }
    }
}

pub fn recommended(works: &[&Work]) -> Markup {
    html! {
        @if works.is_empty() {
            p { "推荐作品即将更新。" }
        } @else {
            @for (index, work) in works.iter().enumerate() {
                div.recommendation-panel data-recommendation-panel=(work.id) hidden[index != 0] {
                    (spotlight(work))
                }
            }
            div.selection-area {
                div.selections aria-label="推荐作品" {
                    @for (index, work) in works.iter().enumerate() {
                        button.selection.theme-shadow-decoration type="button" data-recommend-work=(work.id)
                            aria-pressed=(if index == 0 { "true" } else { "false" }) {
                            span.info { span { (work.plain_author_string()) " / " (work.plain_submission_date_day()) } }
                            span.game-name { span { (work.name) } }
                        }
                    }
                }
            }
        }
    }
}

pub fn dialog(works: &[&Work]) -> Markup {
    html! {
        dialog #recent-work-focused aria-label="作品详情" {
            button.dialog-close type="button" data-close-dialog aria-label="关闭作品详情" { "×" }
            div.dialog-content {}
        }
        @for work in works {
            template id=(format!("work-{}", work.id)) {
                (spotlight(work))
            }
        }
    }
}
