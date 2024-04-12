use std::{rc::Rc, vec};

use sycamore::prelude::*;
use website_model::work::{Platform, Work};

use crate::{
    components::SvgCode,
    svg,
    tool::statics,
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
    pub select_item: &'a Signal<Vec<(usize, String, String, String)>>,
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
        // Determine whether there is work.
        if self.works.get().len() > 0 {
            self.set_work(&self.works.get()[*self.game_index.get()])
        }


    }
    pub fn set_focused_image(&self, index: usize) {
        let len = self.screenshots.get().len();
        self.image_index.set((index + len) % len);//todo fix condition: len = 0
        self.focused_image.set(
            self.screenshots
                .get()
                .get(index.clone())
                .unwrap()
                .1
                .to_owned(),
        );
    }

    pub fn set_work_index(&self, index: usize) {
        self.game_index
            .set(index);
    }

    pub fn refresh_select_list(&self) {
        let mut vec: Vec<(usize, String, String, String)> = vec![];
        for i in 0..self.works.get().len() {
            vec.push((i.clone(), self.works.get()[i].name.clone(), self.works.get()[i].plain_author_string(), self.works.get()[i].submission_date.date_rfc3339[0..=9].to_string()));
        }
        self.select_item.set(vec);
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
            select_item: create_signal(cx, vec![]),
            works: create_signal(cx, (*props.works).clone()),
        },
    );

    //---- Work Change ---------------------
    context.get().refresh_work();
    context.get().refresh_select_list();
    //---- View -------------------------

    view! { cx,
        // Determine whether there is work.
        (if context.get().works.get().len() > 0 {
            view! { cx,
                // Spotlight
                div(class="work-spotlight"){
                    // Screenshots
                    div(class="container"){
                        div(class="content")
                        {
                            div(class="game-image current_image"){
                                img(src=(statics(context.get().focused_image.get().as_str())))
                            }
                        }
                    }
                    // Gallery
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
                    // Author
                    div(class="author"){
                        p() {
                            (format!("By {}", context.get().authors.get()))
                        }
                    }
                    // Description
                    div(class="description"){
                        p() {
                            (context.get().introduce.get())
                        }
                    }
                    // Links
                    div(class="links"){
                        Indexed(
                            iterable=context.get().platforms,
                            view=|cx, it| view!{ cx,
                                div(class="link"){
                                    a(href=&it.url){
                                        (it.platform_type.display_name())
                                        SvgCode(class="link-icon", code=svg::LINK_ARROW)
                                    }
                                }
                            }
                        )
                    }
                }
                
                // Select Area
                div(class="select-area"){
                    SvgCode(class="upper-left-frame", code=svg::UPPER_LEFT_FRAME)
                    SvgCode(class="lower-right-frame", code=svg::LOWER_RIGHT_FRAME)
                    // Options
                    div(class="options"){
                        Indexed(
                            iterable=context.get().select_item,
                            view= move |cx, it| {
                                let index = it.0.clone();
                                view! { cx,
                                    div(class="selection", on:click= move |_|{
                                        context.get().set_work_index(index);
                                        context.get().refresh_work();
                                    }){
                                        div(class="info"){
                                            p(){
                                                (format!("{}/{}", it.2.clone(), it.3.clone()))
                                            }
                                        }
                                        div(class="game-name"){
                                            p(){
                                                (it.1.clone())
                                            }
                                        }
                                        div(class="devide-line"){
                                            div(class="rectangle"){}
                                        }
                                    }
                                }
                            }
                        ) 
                    }
                }
            }
        } else {
            view! { cx, }
        })
    }
}
