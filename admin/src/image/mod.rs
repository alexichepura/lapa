cfg_if::cfg_if! {if #[cfg(feature = "ssr")] {
    mod image_convert;
    pub use image_convert::{create_image_variants, ConvertSettings};
}}
mod image_error;
mod image_path;
pub use image_error::ImageError;
pub use image_path::*;
