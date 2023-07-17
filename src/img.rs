use leptos::*;
use serde::{Deserialize, Serialize};

use crate::settings::use_settings;

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImgData {
    pub id: String,
    pub alt: String,
}

#[component]
pub fn Thumb(cx: Scope, image: ImgData) -> impl IntoView {
    let settings = use_settings(cx);

    let src = format!("/img/{}-s.webp", image.id);
    let srcset = format!("/img/{}-s2.webp 2x", image.id);
    view! { cx, <img
        src=src
        srcset=srcset
        width=settings.thumb_width
        height=settings.thumb_height
        alt=image.alt
    /> }
}

pub fn img_path_small(id: &String) -> String {
    format!("img/{}-s.webp", id)
}
pub fn img_path_small_retina(id: &String) -> String {
    format!("img/{}-s2.webp", id)
}
pub fn img_path_large(id: &String) -> String {
    format!("img/{}-l.webp", id)
}
pub fn img_path_large_retina(id: &String) -> String {
    format!("img/{}-l2.webp", id)
}
pub fn img_url_small(id: &String) -> String {
    format!("/img/{}-s.webp", id)
}
pub fn img_url_small_retina(id: &String) -> String {
    format!("/img/{}-s2.webp", id)
}
pub fn img_url_large(id: &String) -> String {
    format!("/img/{}-l.webp", id)
}
pub fn img_url_large_retina(id: &String) -> String {
    format!("/img/{}-l2.webp", id)
}
pub fn srcset_small(id: &String) -> String {
    let small_retina = img_url_small_retina(&id);
    format!("{small_retina} 2x")
}
pub fn srcset_large(id: &String) -> String {
    let large_retina = img_url_large_retina(&id);
    format!("{large_retina} 2x")
}
