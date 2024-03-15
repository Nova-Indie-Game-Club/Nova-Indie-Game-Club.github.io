use sycamore::prelude::*;


pub mod work;
pub mod footer;

#[derive(Prop)]
pub struct SvgCodeProps{
    code: &'static str,
    #[builder(default)]
    class: &'static str
}

#[component]
pub fn SvgCode<G: Html>(cx: Scope, prop: SvgCodeProps) -> View<G>{
    view!{ cx,
        div(dangerously_set_inner_html=prop.code, class=prop.class)
    }
}