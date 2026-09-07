use maud::{html, Markup, DOCTYPE};

pub mod about;
pub mod content;
pub mod index;

pub fn page(title: &str, prefix: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="zh-CN" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="description" content="Nova 独游社：独立游戏创作、交流与作品展示。";
                title { (title) }
                @for stylesheet in ["root.css", "page/home.css", "recommended_works.css", "recent_work_focused.css", "interactions.css"] {
                    link rel="stylesheet" href=(format!("{prefix}static/css/{stylesheet}"));
                }
                link rel="icon" type="image/png" href=(format!("{prefix}static/assets/images/logo_small.png"));
                script src=(format!("{prefix}static/js/site.js")) defer {}
            }
            body {
                div #root { (body) }
            }
        }
    }
}
