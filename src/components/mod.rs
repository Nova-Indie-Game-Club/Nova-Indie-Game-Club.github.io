use sycamore::{builder, prelude::*};

use crate::tool::statics;

pub mod work;
pub mod footer;


#[derive(Prop)]
pub struct SvgProps{
    path: &'static str,
    #[builder(default)]
    class: &'static str
}
#[component]
pub fn Svg<G: Html>(cx: Scope, prop: SvgProps) -> View<G>{
    view!{ cx, 
        object(data=(statics(prop.path)), class=(prop.class))
    }
}