use maud::{html, Markup, PreEscaped};

pub mod footer;
pub mod work;

/// Only compile-time SVG constants belong here; content data is escaped by Maud.
pub fn link_icon() -> Markup {
    html! { (PreEscaped(crate::svg::LINK_ARROW)) }
}
