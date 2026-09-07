use maud::{html, Markup};

pub fn render() -> Markup {
    super::page(
        "关于 Nova 独游社",
        "../",
        html! {
            main.about-page {
                a href="../" { "← 返回主页" }
                h1 { "关于 Nova 独游社" }
                @for (_, title, text) in super::content::ABOUT_SECTIONS {
                    section {
                        h2 { (title) }
                        p { (text) }
                    }
                }
            }
            (crate::components::footer::render())
        },
    )
}
