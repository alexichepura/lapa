#[derive(Debug, Clone)]
pub struct ImageConfig {
    pub image_upload_path: String,
    pub image_convert_path: String,
}
impl ImageConfig {
    pub fn content_image_upload_path(&self) -> String {
        format!("{}/content", self.image_upload_path)
    }
    pub fn content_image_convert_path(&self) -> String {
        format!("{}/content", self.image_convert_path)
    }
    pub fn content_image_upload_name_ext(&self, name: &str, ext: &str) -> String {
        format!("{}/{}.{}", self.content_image_upload_path(), name, ext)
    }
    pub fn product_image_upload_path(&self) -> String {
        format!("{}/product", self.image_upload_path)
    }
    pub fn product_image_convert_path(&self) -> String {
        format!("{}/product", self.image_convert_path)
    }
    pub fn product_image_upload_name_ext(&self, name: &str, ext: &str) -> String {
        format!("{}/{}.{}", self.product_image_upload_path(), name, ext)
    }
    pub fn post_hero_upload_path(&self) -> String {
        format!("{}/post_hero", self.image_upload_path)
    }
    pub fn post_hero_convert_path(&self) -> String {
        format!("{}/post_hero", self.image_convert_path)
    }
    pub fn post_hero_upload_name_ext(&self, name: &str, ext: &str) -> String {
        format!("{}/{}.{}", self.post_hero_upload_path(), name, ext)
    }
}
