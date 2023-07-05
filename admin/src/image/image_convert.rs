use image::{imageops::FilterType, DynamicImage, ImageError, ImageFormat};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::settings::SettingsImages;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageConvertConfig {
    // img: DynamicImage,
    path: String,
    height: u32,
    width: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConvertSettings {
    pub hero_width: u32,
    pub hero_height: u32,
    pub thumb_width: u32,
    pub thumb_height: u32,
}

impl From<&SettingsImages> for ConvertSettings {
    fn from(data: &SettingsImages) -> Self {
        ConvertSettings {
            hero_width: data.hero_width as u32,
            hero_height: data.hero_height as u32,
            thumb_width: data.thumb_width as u32,
            thumb_height: data.thumb_height as u32,
        }
    }
}

pub type ArcDynamicImage = Arc<DynamicImage>;

pub fn create_image_variant(
    img_decoded: ArcDynamicImage,
    conf: ImageConvertConfig,
) -> Result<(), ImageError> {
    let variant = img_decoded.resize_to_fill(conf.width, conf.height, FilterType::Lanczos3);

    variant.save_with_format(conf.path, ImageFormat::WebP)
}

pub fn create_image_variants(dynamic_image: DynamicImage, settings: &ConvertSettings, id: String) {
    let img_path = "img";
    let conf_large = ImageConvertConfig {
        path: format!("{img_path}/{id}-l.webp"),
        width: settings.hero_width,
        height: settings.hero_height,
    };
    let conf_large_retina = ImageConvertConfig {
        path: format!("{img_path}/{id}-l2.webp"),
        width: settings.hero_width * 2,
        height: settings.hero_height * 2,
    };
    let conf_small = ImageConvertConfig {
        path: format!("{img_path}/{id}-s.webp"),
        width: settings.thumb_width,
        height: settings.thumb_height,
    };
    let conf_small_retina = ImageConvertConfig {
        path: format!("{img_path}/{id}-s2.webp"),
        width: settings.thumb_width * 2,
        height: settings.thumb_height * 2,
    };

    let arc: ArcDynamicImage = Arc::new(dynamic_image);
    let arc_clone1 = arc.clone();
    let arc_clone2 = arc.clone();
    let arc_clone3 = arc.clone();
    let arc_clone4 = arc.clone();

    let hadle_large = std::thread::spawn(move || {
        let _result = create_image_variant(arc_clone1, conf_large);
    });
    let hadle_large_retina = std::thread::spawn(move || {
        let _result = create_image_variant(arc_clone2, conf_large_retina);
    });
    let hadle_small = std::thread::spawn(move || {
        let _result = create_image_variant(arc_clone3, conf_small);
    });
    let hadle_small_retina = std::thread::spawn(move || {
        let _result = create_image_variant(arc_clone4, conf_small_retina);
    });

    hadle_large.join().unwrap();
    hadle_large_retina.join().unwrap();
    hadle_small.join().unwrap();
    hadle_small_retina.join().unwrap();
}
