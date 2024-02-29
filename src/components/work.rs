use std::{ops::Deref, rc::Rc, vec};

use sycamore::prelude::*;
use website_model::work::{Platform, Work};

use crate::{components::Svg, tool::{link_in_new_tab, statics}};

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

fn set_work(
    work: &Work,
    name: &Signal<String>,
    introduce: &Signal<String>,
    platforms: &Signal<Vec<Platform>>,
    authors: &Signal<String>,
    screenshots: &Signal<Vec<(usize, String)>>,
    cover: &Signal<String>,
) {
    name.set(work.name.clone());
    introduce.set(work.introduce.clone());
    platforms.set(work.platforms.clone());
    authors.set(work.plain_author_string());
    let mut vec: Vec<(usize, String)> = vec![];
    for i in 0..work.screenshots.len().min(5usize){
        vec.push((i, work.screenshots[i].clone()))
    }
    screenshots.set(vec);
    cover.set(work.cover.clone().unwrap_or_default());
}

fn set_focused_image(index: usize, image_index: &Signal<usize>, focused_image: &Signal<String>, screenshots: &Signal<Vec<(usize, String)>>){
    let len = screenshots.get().len();
    image_index.set((index + len) % len);
    focused_image.set(screenshots.get().get(index.clone()).unwrap().1.to_owned());
}


#[component]
pub fn FocusedWorkPanel<G: Html>(cx: Scope, props: FocusedWorkPanelProps) -> View<G> {
    let name = create_signal(cx, "Game name here".to_string());
    let introduce = create_signal(cx, "Introduce here".to_string());
    let platforms: &Signal<Vec<Platform>> = create_signal(cx, vec![]);
    let authors: &Signal<String> = create_signal(cx, "authors".to_string());
    let screenshots: &Signal<Vec<(usize,String)>> = create_signal(cx, vec![]);
    let focused_image = create_signal(cx, "".to_string());
    let cover = create_signal(cx, "".to_string());

    //---- Work Change ---------------------
    let game_index = create_signal(cx, 0usize);
    let rec_works_count = props.works.len();
    let rec_works_rc_clone_0 = props.works.clone();
    let rec_works_rc_clone_1 = props.works.clone();

    set_work(&props.works[*game_index.get()], name, introduce, platforms, authors, screenshots, cover);

    let on_click_previous = move |_|{
        game_index.set((*game_index.get() + rec_works_count - 1) % rec_works_count);
        set_work(&rec_works_rc_clone_0[*game_index.get()], name, introduce, platforms, authors, screenshots, cover);
    };
    let on_click_next = move |_|{
        game_index.set((*game_index.get() + rec_works_count + 1) % rec_works_count);
        set_work(&rec_works_rc_clone_1[*game_index.get()], name, introduce, platforms, authors, screenshots, cover);
    };
    
    //---- Image Change -----------------
    let image_index = create_signal(cx, 0usize);


    //---- View -------------------------

    view! { cx,
        div(class="work-container"){
            div(class="image-panel"){
                div(class="stage"){
                    div(on:click=on_click_previous,id="recommended-works-previous-game-button"){
                        Svg(path="assets/svg/left_tri_arrow.svg")
                    }
                    div(class="focused-image"){
                        img(src=(statics(focused_image.get().as_str()))) //todo test
                    }
                    div(on:click=on_click_next,id="recommended-works-next-game-button"){
                        Svg(path="assets/svg/right_tri_arrow.svg")
                    }
                }
                div(class="gallery"){
                    Indexed(
                        iterable=screenshots,
                        view=|cx, it| {
                            let index = it.0.clone();
                            view! { cx, 
                                // div(class="item", on:click= move |_|{
                                //     set_focused_image(index, image_index, focused_image, screenshots)
                                // }){
                                //     img(src=(statics(it.1.as_str())))
                                // }
                            }
                        }
                    )
                }
            }
            div(class="info-panel"){
                //column
                h3{
                    (name.get())
                }
                div(class="detail"){
                    //row
                    div(class="left-part"){
                        //column
                        div(class="cover"){
                            img(src=(statics(cover.get().as_str())), alt="cover")
                        }
                        div(class="platform"){
                            Indexed(
                                iterable=platforms,
                                view=|cx, it| view!{ cx,
                                    div(class="link-button", onclick=link_in_new_tab(&it.url)){
                                        //row
                                        div(class="text"){
                                            (it.platform_type.display_name())
                                        }
                                        Svg(class="link-icon", path="assets/svg/arrow.svg")
                                    }
                                }
                            )
                        }
                    }
                    div(class="right-part"){
                        //column
                        p(class="text") {
                            (introduce.get())
                        }
                        p(class="author") {
                            (format!("作者: {}", authors.get()))
                        }
                    }
                }
            }
        }
    }
}
