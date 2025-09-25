use crate::ffi;

pub fn calc_canvas_geometry(
    src_width: i32,
    src_height: i32,
    font_ratio: f32,
    zoom: bool,
    stretch: bool,
) -> (i32, i32) {
    let mut width: i32 = 0;
    let mut height: i32 = 0;

    unsafe {
        ffi::chafa_calc_canvas_geometry(
            src_width,
            src_height,
            &mut width,
            &mut height,
            font_ratio,
            if zoom { 1 } else { 0 },
            if stretch { 1 } else { 0 },
        );
    }
    (width, height)
}

pub const CHAFA_MAJOR_VERSION: u32 = ffi::CHAFA_MAJOR_VERSION;
pub const CHAFA_MICRO_VERSION: u32 = ffi::CHAFA_MICRO_VERSION;
pub const CHAFA_MINOR_VERSION: u32 = ffi::CHAFA_MINOR_VERSION;

#[repr(u32)]
pub enum PixelType {
    /// Premultiplied RGBA, 8 bits per channel.
    RGBA8Premultiplied = ffi::ChafaPixelType_CHAFA_PIXEL_RGBA8_PREMULTIPLIED,
    /// Premultiplied BGRA, 8 bits per channel.
    BGRA8Premultiplied = ffi::ChafaPixelType_CHAFA_PIXEL_BGRA8_PREMULTIPLIED,
    /// Premultiplied ARGB, 8 bits per channel.
    ARGB8Premultiplied = ffi::ChafaPixelType_CHAFA_PIXEL_ARGB8_PREMULTIPLIED,
    /// Premultiplied ABGR, 8 bits per channel.
    ABGR8Premultiplied = ffi::ChafaPixelType_CHAFA_PIXEL_ABGR8_PREMULTIPLIED,

    /// Unassociated RGBA, 8 bits per channel.
    RGBA8Unassociated = ffi::ChafaPixelType_CHAFA_PIXEL_RGBA8_UNASSOCIATED,
    /// Unassociated BGRA, 8 bits per channel.
    BGRA8Unassociated = ffi::ChafaPixelType_CHAFA_PIXEL_BGRA8_UNASSOCIATED,
    /// Unassociated ARGB, 8 bits per channel.
    ARGB8Unassociated = ffi::ChafaPixelType_CHAFA_PIXEL_ARGB8_UNASSOCIATED,
    /// Unassociated ABGR, 8 bits per channel.
    ABGR8Unassociated = ffi::ChafaPixelType_CHAFA_PIXEL_ABGR8_UNASSOCIATED,

    /* 24 bits per pixel */
    /// Packed RGB (no alpha), 8 bits per channel.
    RGB8 = ffi::ChafaPixelType_CHAFA_PIXEL_RGB8,
    /// Packed BGR (no alpha), 8 bits per channel.
    BGR8 = ffi::ChafaPixelType_CHAFA_PIXEL_BGR8,

    /// Last supported pixel type, plus one.
    Max = ffi::ChafaPixelType_CHAFA_PIXEL_MAX,
}

impl From<u32> for PixelType {
    fn from(value: u32) -> Self {
        match value {
            ffi::ChafaPixelType_CHAFA_PIXEL_RGBA8_PREMULTIPLIED => PixelType::RGBA8Premultiplied,
            ffi::ChafaPixelType_CHAFA_PIXEL_BGRA8_PREMULTIPLIED => PixelType::BGRA8Premultiplied,
            ffi::ChafaPixelType_CHAFA_PIXEL_ARGB8_PREMULTIPLIED => PixelType::ARGB8Premultiplied,
            ffi::ChafaPixelType_CHAFA_PIXEL_ABGR8_PREMULTIPLIED => PixelType::ABGR8Premultiplied,
            ffi::ChafaPixelType_CHAFA_PIXEL_RGBA8_UNASSOCIATED => PixelType::RGBA8Unassociated,
            ffi::ChafaPixelType_CHAFA_PIXEL_BGRA8_UNASSOCIATED => PixelType::BGRA8Unassociated,
            ffi::ChafaPixelType_CHAFA_PIXEL_ARGB8_UNASSOCIATED => PixelType::ARGB8Unassociated,
            ffi::ChafaPixelType_CHAFA_PIXEL_ABGR8_UNASSOCIATED => PixelType::ABGR8Unassociated,
            ffi::ChafaPixelType_CHAFA_PIXEL_RGB8 => PixelType::RGB8,
            ffi::ChafaPixelType_CHAFA_PIXEL_BGR8 => PixelType::BGR8,
            ffi::ChafaPixelType_CHAFA_PIXEL_MAX => PixelType::Max,
            _ => PixelType::RGBA8Premultiplied,
        }
    }
}

impl From<PixelType> for u32 {
    fn from(value: PixelType) -> u32 {
        match value {
            PixelType::RGBA8Premultiplied => ffi::ChafaPixelType_CHAFA_PIXEL_RGBA8_PREMULTIPLIED,
            PixelType::BGRA8Premultiplied => ffi::ChafaPixelType_CHAFA_PIXEL_BGRA8_PREMULTIPLIED,
            PixelType::ARGB8Premultiplied => ffi::ChafaPixelType_CHAFA_PIXEL_ARGB8_PREMULTIPLIED,
            PixelType::ABGR8Premultiplied => ffi::ChafaPixelType_CHAFA_PIXEL_ABGR8_PREMULTIPLIED,
            PixelType::RGBA8Unassociated => ffi::ChafaPixelType_CHAFA_PIXEL_RGBA8_UNASSOCIATED,
            PixelType::BGRA8Unassociated => ffi::ChafaPixelType_CHAFA_PIXEL_BGRA8_UNASSOCIATED,
            PixelType::ARGB8Unassociated => ffi::ChafaPixelType_CHAFA_PIXEL_ARGB8_UNASSOCIATED,
            PixelType::ABGR8Unassociated => ffi::ChafaPixelType_CHAFA_PIXEL_ABGR8_UNASSOCIATED,
            PixelType::RGB8 => ffi::ChafaPixelType_CHAFA_PIXEL_RGB8,
            PixelType::BGR8 => ffi::ChafaPixelType_CHAFA_PIXEL_BGR8,
            PixelType::Max => ffi::ChafaPixelType_CHAFA_PIXEL_MAX,
        }
    }
}
