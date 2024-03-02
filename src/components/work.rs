use std::{ops::Deref, rc::Rc, vec};

use sycamore::prelude::*;
use website_model::work::{Platform, Work};

use crate::{
    components::SvgCode,
    svg,
    tool::{link_in_new_tab, statics},
};

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
    pub works: Rc<Vec<Work>>,
}
struct WorkPanelContext<'a> {
    pub name: &'a Signal<String>,
    pub introduce: &'a Signal<String>,
    pub platforms: &'a Signal<Vec<Platform>>,
    pub authors: &'a Signal<String>,
    pub screenshots: &'a Signal<Vec<(usize, String)>>,
    pub cover: &'a Signal<String>,
    pub game_index: &'a Signal<usize>,
    pub image_index: &'a Signal<usize>,
    pub focused_image: &'a Signal<String>,
    pub works: &'a Signal<Vec<Work>>,
}
impl<'a> WorkPanelContext<'a> {
    pub fn set_work(&self, work: &Work) {
        self.name.set(work.name.clone());
        self.introduce.set(work.introduce.clone());
        self.platforms.set(work.platforms.clone());
        self.authors.set(work.plain_author_string());
        let mut vec: Vec<(usize, String)> = vec![];
        for i in 0..work.screenshots.len().min(5usize) {
            vec.push((i, work.screenshots[i].clone()))
        }
        self.screenshots.set(vec);
        self.cover.set(work.cover.clone().unwrap_or_default());
        self.set_focused_image(0usize);
    }
    pub fn refresh_work(&self) {
        self.set_work(&self.works.get()[*self.game_index.get()])
    }
    pub fn set_focused_image(&self, index: usize) {
        let len = self.screenshots.get().len();
        self.image_index.set((index + len) % len);
        self.focused_image.set(
            self.screenshots
                .get()
                .get(index.clone())
                .unwrap()
                .1
                .to_owned(),
        );
    }

    pub fn offset_work_index(&self, offset: i32) {
        let len = self.works.get().len() as i32;
        let offset = offset % len;
        self.game_index
            .set(((*self.game_index.get() as i32 + len + offset) % len) as usize);
    }
}

#[component]
pub fn FocusedWorkPanel<G: Html>(cx: Scope, props: FocusedWorkPanelProps) -> View<G> {
    let context = create_signal(
        cx,
        WorkPanelContext {
            name: create_signal(cx, "Game name here".to_string()),
            introduce: create_signal(cx, "Introduce here".to_string()),
            platforms: create_signal(cx, vec![]),
            authors: create_signal(cx, "authors".to_string()),
            screenshots: create_signal(cx, vec![]),
            focused_image: create_signal(cx, "".to_string()),
            cover: create_signal(cx, "".to_string()),
            game_index: create_signal(cx, 0usize),
            image_index: create_signal(cx, 0usize),
            works: create_signal(cx, (*props.works).clone()),
        },
    );

    //---- Work Change ---------------------
    context.get().refresh_work();

    let on_click_previous = move |_| {
        context.get().offset_work_index(-1);
        context.get().refresh_work();
    };
    let on_click_next = move |_| {
        context.get().offset_work_index(1);
        context.get().refresh_work();
    };

    //---- View -------------------------

    view! { cx,
        div(class="work-container"){
            //column
            div(on:click=on_click_previous,class="arrow",id="recommended-works-previous-game-button"){
                SvgCode(class="arrow-svg", code=svg::LEFT_TRI_ARROW)
            }
            div(class="work-content"){
                div(class="image-panel"){
                    //column
                    div(class="focused-image"){
                        img(src=(statics(context.get().focused_image.get().as_str()))) //todo test
                    }
                    div(class="gallery"){
                        Indexed(
                            iterable=context.get().screenshots,
                            view= move |cx, it| {
                            let index = it.0.clone();
                            view! { cx,
                                div(class="item", on:click= move |_|{
                                    context.get().set_focused_image(index);
                                }){
                                    img(src=(statics(it.1.as_str())))
                                }
                                }
                            }
                        )
                    }
                }
                div(class="info-panel"){
                div(class="detail"){
                    //row
                    div(class="left-part"){
                        //column
                        div(class="cover"){
                            img(src=(statics(context.get().cover.get().as_str())), alt="cover")
                        }
                        div(class="platform"){
                            Indexed(
                                iterable=context.get().platforms,
                                view=|cx, it| view!{ cx,
                                    div(class="link-button", onclick=link_in_new_tab(&it.url)){
                                        //row
                                        div(class="text"){
                                            (it.platform_type.display_name())
                                        }
                                        SvgCode(class="link-icon", code=svg::LINK_ARROW)
                                    }
                                }
                            )
                        }
                    }
                    div(class="right-part"){
                        //column
                        div{
                            h3{
                                (context.get().name.get())
                            }
                            p(class="text") {
                                (context.get().introduce.get())
                            }
                        }
                        p(class="author") {
                            (format!("作者: {}", context.get().authors.get()))
                        }
                    }
                }
            }

            }
            div(on:click=on_click_next,class="arrow",id="recommended-works-next-game-button"){
                SvgCode(class="arrow-svg", code=svg::RIGHT_TRI_ARROW)
            }
        }
    }
}
