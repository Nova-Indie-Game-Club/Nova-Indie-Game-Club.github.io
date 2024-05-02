use std::{rc::Rc, vec};

use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use website_model::work::{Platform, Work};

use crate::{components::SvgCode, svg, templates, tool::statics};

#[derive(Prop)]
pub struct RecentWorkItemProps<'a> {
    pub cover_path: String,
    pub date: String,
    pub author: String,
    pub title: String,
    pub work_id: String,
    pub focused_props: WorkSpotlightProps<'a>,
}
#[component]
pub fn RecentWorkItem<'a, G: Html>(cx: Scope<'a>, props: RecentWorkItemProps<'a>) -> View<G> {
    let on_click = move |_it| {
        props.focused_props.set_work(
            props
                .focused_props
                .works
                .get()
                .iter()
                .find(|it| it.id == props.work_id)
                .unwrap(),
        );
        templates::index::set_recent_work_focused_enable(true);
    };
    view!(cx,
        div(class="item", on:click=on_click){
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SelectionItem {
    index: usize,
    name: String,
    author_text: String,
    data_text: String,
}

fn refresh_select_list(works: &Signal<Vec<Work>>, select_item: &Signal<Vec<SelectionItem>>) {
    let mut vec: Vec<SelectionItem> = vec![];
    for (i, work) in works.get().iter().enumerate() {
        vec.push(SelectionItem {
            index: i,
            name: work.name.clone(),
            author_text: work.plain_author_string(),
            data_text: work.submission_date.date_rfc3339[0..=9].to_string(),
        });
    }
    select_item.set(vec);
}
#[component]
pub fn FocusedWorkPanel<G: Html>(cx: Scope, props: FocusedWorkPanelProps) -> View<G> {
    let name = create_signal(cx, "Game name here".to_string());
    let introduce = create_signal(cx, "Introduce here".to_string());
    let platforms = create_signal(cx, vec![]);
    let authors = create_signal(cx, "authors".to_string());
    let screenshots = create_signal(cx, vec![]);
    let focused_image = create_signal(cx, "".to_string());
    let cover = create_signal(cx, "".to_string());
    let game_index = create_signal(cx, 0usize);
    let image_index = create_signal(cx, 0usize);
    let select_items = create_signal(cx, Vec::<SelectionItem>::new());
    let works = create_signal(cx, (*props.works).clone());

    let work_spotlight_props = WorkSpotlightProps {
        name,
        introduce,
        platforms,
        authors,
        screenshots,
        focused_image,
        cover,
        image_index,
        works,
    };

    //---- Work Change ---------------------
    work_spotlight_props.refresh_work(*game_index.get());
    refresh_select_list(works, select_items);
    //---- View -------------------------

    view! { cx,
        // Determine whether there is work.
        (if works.get().len() > 0 {
            view! { cx,
                WorkSpotlight(work_spotlight_props)
                // Select Area
                div(class="select-area"){
                    SvgCode(class="upper-left-frame", code=svg::UPPER_LEFT_FRAME)
                    SvgCode(class="lower-right-frame", code=svg::LOWER_RIGHT_FRAME)
                    // Options
                    div(class="options"){
                        Indexed(
                            iterable=select_items,
                            view= move |cx, it| {
                                let index = it.index;
                                view! { cx,
                                    div(class="selection", on:click= move |_|{
                                        game_index.set(index);
                                        if works.get().len() > 0 {
                                            work_spotlight_props.set_work(&works.get()[*game_index.get()])
                                        }
                                    }){
                                        div(class="info"){
                                            p(){
                                                (format!("{}/{}", &it.author_text, &it.data_text))
                                            }
                                        }
                                        div(class="game-name"){
                                            p(){
                                                (it.name.clone())
                                            }
                                        }
                                        div(class="divide-line"){
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

#[derive(Prop, Clone, Copy)]
pub struct WorkSpotlightProps<'a> {
    pub name: &'a Signal<String>,
    pub introduce: &'a Signal<String>,
    pub platforms: &'a Signal<Vec<Platform>>,
    pub authors: &'a Signal<String>,
    pub screenshots: &'a Signal<Vec<(usize, String)>>,
    pub cover: &'a Signal<String>,
    pub image_index: &'a Signal<usize>,
    pub focused_image: &'a Signal<String>,
    pub works: &'a Signal<Vec<Work>>,
}

impl<'a> WorkSpotlightProps<'a> {
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
    pub fn set_focused_image(&self, index: usize) {
        if self.has_screenshots() {
            let len = self.screenshots.get().len();
            self.image_index.set((index + len) % len); //todo fix condition: len = 0
            self.focused_image
                .set(self.screenshots.get().get(index).cloned().unwrap().1);
        } else {
            self.focused_image.set((*self.cover.get()).clone());
        }
    }
    pub fn has_screenshots(&self) -> bool {
        self.screenshots.get().len() > 0
    }
    pub fn refresh_work(&self, index: usize) {
        // Determine whether there is work.
        if self.works.get().len() > 0 {
            self.set_work(&self.works.get()[index])
        }
    }
}

#[component]
pub fn WorkSpotlight<'a, G: Html>(cx: Scope<'a>, props: WorkSpotlightProps<'a>) -> View<G> {
    view! { cx,
    // Spotlight
    div(class="work-spotlight"){
        // Screenshots
        div(class="container"){
            div(class="content")
            {
                div(class="game-image current_image"){
                    img(src=(statics(props.focused_image.get().as_str())))
                }
            }
        }
        // Gallery
        div(class="gallery"){
            (if props.has_screenshots() {
                view!{ cx,
                    Indexed(
                        iterable=props.screenshots,
                        view= move |cx, it| {
                        let index = it.0;
                        view! { cx,
                            div(class="item", on:click= move |_|{
                                props.set_focused_image(index);
                            }){
                                img(src=(statics(it.1.as_str())))
                            }
                            }
                        }
                    )
                }
            } else {
                view!(cx,)
            })
        }


        // Author
        div(class="author"){
            p() {
                (format!("By {}", props.authors.get()))
            }
        }
        // Description
        div(class="description"){
            p() {
                (props.introduce.get())
            }
        }
        // Links
        div(class="links"){
            Indexed(
                iterable=props.platforms,
                view=|cx, it| view!{ cx,
                    div(class="link"){
                        a(href=&it.url, target="_blank"){
                            (it.platform_type.display_name())
                            SvgCode(class="link-icon", code=svg::LINK_ARROW)
                        }
                    }
                }
            )
        }
    }
    }
}
