use crate::{CanvasMode, Passthrough, PixelMode, ffi};

pub struct TermInfo {
    pub raw: *mut ffi::ChafaTermInfo,
}

impl TermInfo {
    pub fn new() -> Result<Self, &'static str> {
        let raw = unsafe { ffi::chafa_term_info_new() };
        if raw.is_null() {
            Err("Chafa -> Failed to create TermInfo")
        } else {
            Ok(Self { raw })
        }
    }

    pub fn detect() -> Result<Self, &'static str> {
        let raw = unsafe {
            let envp = ffi::g_get_environ();
            ffi::chafa_term_db_detect(ffi::chafa_term_db_get_default(), envp)
        };
        if raw.is_null() {
            Err("Chafa -> Failed to create TermInfo")
        } else {
            Ok(Self { raw })
        }
    }

    pub fn get_best_canvas_mode(&self) -> CanvasMode {
        unsafe { CanvasMode::from(ffi::chafa_term_info_get_best_canvas_mode(self.raw)) }
    }

    pub fn get_best_pixel_mode(&self) -> PixelMode {
        unsafe { PixelMode::from(ffi::chafa_term_info_get_best_pixel_mode(self.raw)) }
    }

    pub fn get_is_pixel_passthrough_needed(&self, pixel_mode: PixelMode) -> Passthrough {
        unsafe {
            Passthrough::from(ffi::chafa_term_info_get_is_pixel_passthrough_needed(
                self.raw,
                pixel_mode as u32,
            ) as u32)
        }
    }
}

impl Drop for TermInfo {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ffi::chafa_term_info_unref(self.raw);
            }
        }
    }
}
