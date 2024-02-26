use sycamore::prelude::*;
use website_model::work::Work;

use crate::statics;


#[derive(Prop)]
pub struct RecentWorkItemProps {
    pub cover_path: String,
    pub date: String,
    pub author: String,
    pub title: String,
}
#[component]
pub fn RecentWorkItem<G: Html>(cx: Scope, props: RecentWorkItemProps) -> View<G> {
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

#[derive(Prop)]
pub struct FocusedWorkPanelProps {
    pub work: Work,
}
#[component]
pub fn FocusedWorkPanel<G: Html>(cx: Scope, props: FocusedWorkPanelProps) -> View<G>{
    view!{ cx,

    }
}