use crate::config::Config;
use crate::ffi;
use crate::misc::PixelType;
use crate::term_info::TermInfo;

pub struct Canvas {
    raw: *mut ffi::ChafaCanvas,
    term_info: TermInfo,
}

impl Canvas {
    pub fn new(config: Config) -> Result<Self, &'static str> {
        let raw = unsafe { ffi::chafa_canvas_new(config.raw) };
        if raw.is_null() {
            Err("Chafa -> Failed to create canvas.")
        } else {
            let ti = TermInfo::detect();
            match ti {
                Ok(term_info) => Ok(Self { raw, term_info }),
                Err(e) => Err(e),
            }
        }
    }

    pub fn set_pixels(
        &self,
        pixels: &[u8],
        pixel_type: PixelType,
        src_width: i32,
        src_height: i32,
        src_rowstride: i32,
    ) {
        unsafe {
            ffi::chafa_canvas_draw_all_pixels(
                self.raw,
                pixel_type as u32,
                pixels.as_ptr(),
                src_width,
                src_height,
                src_rowstride,
            );
        }
    }

    pub fn into_string(&self) -> Result<String, &'static str> {
        let mut term: *mut ffi::ChafaTermInfo = std::ptr::null_mut();

        if !self.term_info.raw.is_null() {
            term = self.term_info.raw;
        }
        let g_str = unsafe { ffi::chafa_canvas_print(self.raw, term) };
        if g_str.is_null() {
            return Err("Chafa -> Failed to retrieve GString from `chafa_canvas_print()`");
        }
        let str = unsafe { std::ffi::CStr::from_ptr((*g_str).str_ as *const std::os::raw::c_char) };
        unsafe {
            ffi::g_string_free(g_str, 1);
        }
        Ok(str.to_string_lossy().into_owned())
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ffi::chafa_canvas_unref(self.raw);
            }
        }
    }
}
