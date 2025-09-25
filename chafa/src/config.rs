use crate::ffi::{self, chafa_canvas_config_unref};

pub struct Config {
    pub raw: *mut ffi::ChafaCanvasConfig,
}

impl Config {
    pub fn new() -> Result<Self, &'static str> {
        let raw = unsafe { ffi::chafa_canvas_config_new() };
        if raw.is_null() {
            Err("Chafa -> Failed to create Config")
        } else {
            Ok(Self { raw })
        }
    }

    // pub fn detect() -> Result<Self, &'static str> {}
}

impl Drop for Config {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                chafa_canvas_config_unref(self.raw);
            }
        }
    }
}
