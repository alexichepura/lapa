use image::{imageops::FilterType, DynamicImage, ImageError, ImageFormat};
use std::sync::Arc;

use crate::settings::SettingsImages;

use super::{img_path_large, img_path_large_retina, img_path_small, img_path_small_retina};

#[derive(Clone, Debug)]
pub struct ImageConvertConfig {
    img: Arc<DynamicImage>,
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

pub fn create_image_variant(conf: ImageConvertConfig) -> Result<(), ImageError> {
    let variant = conf
        .img
        .resize_to_fill(conf.width, conf.height, FilterType::Lanczos3);

    variant.save_with_format(conf.path, ImageFormat::WebP)
}

pub fn create_image_variants(dynamic_image: DynamicImage, settings: &ConvertSettings, id: String) {
    let arc = Arc::new(dynamic_image);
    let conf_large = ImageConvertConfig {
        img: arc.clone(),
        path: img_path_large(&id),
        width: settings.hero_width,
        height: settings.hero_height,
    };
    let conf_large_retina = ImageConvertConfig {
        img: arc.clone(),
        path: img_path_large_retina(&id),
        width: settings.hero_width * 2,
        height: settings.hero_height * 2,
    };
    let conf_small = ImageConvertConfig {
        img: arc.clone(),
        path: img_path_small(&id),
        width: settings.thumb_width,
        height: settings.thumb_height,
    };
    let conf_small_retina = ImageConvertConfig {
        img: arc.clone(),
        path: img_path_small_retina(&id),
        width: settings.thumb_width * 2,
        height: settings.thumb_height * 2,
    };

    let hadle_large = std::thread::spawn(move || {
        let _result = create_image_variant(conf_large);
    });
    let hadle_large_retina = std::thread::spawn(move || {
        let _result = create_image_variant(conf_large_retina);
    });
    let hadle_small = std::thread::spawn(move || {
        let _result = create_image_variant(conf_small);
    });
    let hadle_small_retina = std::thread::spawn(move || {
        let _result = create_image_variant(conf_small_retina);
    });

    hadle_large.join().unwrap();
    hadle_large_retina.join().unwrap();
    hadle_small.join().unwrap();
    hadle_small_retina.join().unwrap();
}
