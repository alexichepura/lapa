use image::{DynamicImage, ImageError};
use serde::{Deserialize, Serialize};

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

pub fn create_image_variants(img_decoded: &DynamicImage, settings: ConvertSettings, id: String) {
    let img_path = "img";

    // TODO process in threads

    create_image_variant(
        img_decoded,
        ImageSize {
            width: settings.hero_width,
            height: settings.hero_height,
        },
        format!("{img_path}/{id}-l.webp"),
    );
    create_image_variant(
        img_decoded,
        ImageSize {
            width: settings.thumb_width,
            height: settings.thumb_height,
        },
        format!("{img_path}/{id}-s.webp"),
    );
}
