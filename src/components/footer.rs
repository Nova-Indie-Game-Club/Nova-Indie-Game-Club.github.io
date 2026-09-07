use maud::{html, Markup};

pub fn render() -> Markup {
    html! {
        footer {
            div.footer-left-part {
                div.footer-item.footer-item-layout {
                    div.up-part {
                        div.left-part { div.item-title { "联系我们" } }
                        div.right-part { a.item-text href="mailto:hi@novaindiegame.club" { "hi@novaindiegame.club" } }
                    }
                }
                a.footer-item.has-link.footer-item-layout
                    href="https://github.com/Nova-Indie-Game-Club/Nova-Indie-Game-Club.github.io"
                    target="_blank" rel="noopener noreferrer" {
                    div.up-part {
                        div.left-part { div.item-title { "网站信息" } }
                        div.right-part {
                            div.item-text { "网站仓库" }
                            (super::link_icon())
                        }
                    }
                    div.down-part {
                        p { "网站使用 Rust 和 Maud 生成，代码遵循 MIT 开源协议，资源遵循 CC BY-NC-SA 4.0 协议。" }
                    }
                }
                div.license.footer-item-layout {
                    p {
                        "Nova Website's assets files by Nova Indie Game Club are licensed under "
                        a href="https://creativecommons.org/licenses/by-nc-sa/4.0/"
                            target="_blank" rel="license noopener noreferrer" { "CC BY-NC-SA 4.0" }
                        "."
                    }
                }
            }
        }
    }
}
