#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CdnImageFormat {
    Avif,
    Webp,
    Jpeg,
}
impl CdnImageFormat {
    pub const VALUES: [Self; 3] = [Self::Avif, Self::Webp, Self::Jpeg];

    pub fn from_accept_types(accept_types: &str) -> Self {
        let cdn_format = if accept_types.contains("image/avif,") {
            CdnImageFormat::Avif
        } else if accept_types.contains("image/webp,") {
            CdnImageFormat::Webp
        } else if accept_types.contains("image/jpeg,") || accept_types.contains("image/*,") {
            CdnImageFormat::Jpeg
        } else {
            //     Err(StatusCode::UNSUPPORTED_MEDIA_TYPE)
            CdnImageFormat::Jpeg
        };
        return cdn_format;
    }
}
impl std::fmt::Display for CdnImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CdnImageFormat::Avif => "avif",
                CdnImageFormat::Webp => "webp",
                CdnImageFormat::Jpeg => "jpeg",
            }
        )
    }
}
// https://open-props.style/#media-queries
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CdnImageSize {
    XL,
    L,
    M,
    S,
}
impl CdnImageSize {
    pub const VALUES: [Self; 4] = [Self::S, Self::M, Self::L, Self::XL];
    pub fn to_width(&self) -> u32 {
        match self {
            CdnImageSize::XL => 1440,
            CdnImageSize::L => 1024,
            CdnImageSize::M => 768,
            CdnImageSize::S => 480,
        }
    }
}
impl TryFrom<&str> for CdnImageSize {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "xl" => Self::XL,
            "l" => Self::L,
            "m" => Self::M,
            "s" => Self::S,
            _ => Err(())?,
        })
    }
}
impl std::fmt::Display for CdnImageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CdnImageSize::XL => "xl",
                CdnImageSize::L => "l",
                CdnImageSize::M => "m",
                CdnImageSize::S => "s",
            }
        )
    }
}
