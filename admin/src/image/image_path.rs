pub fn img_path_small(id: &String) -> String {
    format!("/img/{}-s.webp", id)
}
pub fn img_path_small_retina(id: &String) -> String {
    format!("/img/{}-s2.webp", id)
}
pub fn img_path_large(id: &String) -> String {
    format!("/img/{}-l.webp", id)
}
pub fn img_path_large_retina(id: &String) -> String {
    format!("/img/{}-l2.webp", id)
}
