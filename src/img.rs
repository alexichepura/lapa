use leptos::*;
use serde::{Deserialize, Serialize};

use crate::app::SettingsCx;

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImgData {
    pub id: String,
    pub alt: String,
}

#[component]
pub fn Thumb(cx: Scope, image: ImgData) -> impl IntoView {
    let settings = use_context::<SettingsCx>(cx).expect("to have found the settings provided");

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
