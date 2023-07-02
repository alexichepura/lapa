use image::{DynamicImage, ImageError};
use serde::{Deserialize, Serialize};

use crate::settings::SettingsImages;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageSize {
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

pub fn create_image_variant(
    img_decoded: &DynamicImage,
    size: ImageSize,
    path: String,
) -> Result<(), ImageError> {
    let variant = img_decoded.resize_to_fill(
        size.width,
        size.height,
        image::imageops::FilterType::Lanczos3,
    );

    variant.save_with_format(path, image::ImageFormat::WebP)
}

pub fn create_image_variants(dynamic_image: &DynamicImage, settings: &ConvertSettings, id: String) {
    let img_path = "img";

    // TODO process in threads

    create_image_variant(
        dynamic_image,
        ImageSize {
            width: settings.hero_width,
            height: settings.hero_height,
        },
        format!("{img_path}/{id}-l.webp"),
    )
    .unwrap();
    create_image_variant(
        dynamic_image,
        ImageSize {
            width: settings.hero_width * 2,
            height: settings.hero_height * 2,
        },
        format!("{img_path}/{id}-l2.webp"),
    )
    .unwrap();

    create_image_variant(
        dynamic_image,
        ImageSize {
            width: settings.thumb_width,
            height: settings.thumb_height,
        },
        format!("{img_path}/{id}-s.webp"),
    )
    .unwrap();
    create_image_variant(
        dynamic_image,
        ImageSize {
            width: settings.thumb_width * 2,
            height: settings.thumb_height * 2,
        },
        format!("{img_path}/{id}-s2.webp"),
    )
    .unwrap();
}
