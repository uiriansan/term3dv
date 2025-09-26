use crate::{ffi, term_info::TermInfo};
use crossterm::terminal;

pub struct Config {
    pub raw: *mut ffi::ChafaCanvasConfig,
    pub term_info: TermInfo,
}

impl Config {
    pub fn new() -> Result<Self, &'static str> {
        let raw = unsafe { ffi::chafa_canvas_config_new() };
        if raw.is_null() {
            Err("Chafa -> Failed to create Config")
        } else {
            let ti = TermInfo::detect();
            match ti {
                Ok(term_info) => Ok(Self { raw, term_info }),
                Err(e) => Err(e),
            }
        }
    }

    pub fn set_canvas_mode(&self, mode: CanvasMode) {
        unsafe {
            ffi::chafa_canvas_config_set_canvas_mode(self.raw, mode as u32);
        }
    }

    pub fn set_pixel_mode(&self, mode: PixelMode) {
        unsafe {
            ffi::chafa_canvas_config_set_pixel_mode(self.raw, mode as u32);
        }
    }

    pub fn set_passthrough(&self, passthrough: Passthrough) {
        unsafe {
            ffi::chafa_canvas_config_set_passthrough(self.raw, passthrough as u32);
        }
    }

    pub fn set_geometry(&self, width_cells: i32, height_cells: i32) {
        unsafe {
            ffi::chafa_canvas_config_set_geometry(self.raw, width_cells, height_cells);
        }
    }

    pub fn set_cell_geometry(&self, cell_width: i32, cell_height: i32) {
        unsafe {
            ffi::chafa_canvas_config_set_cell_geometry(self.raw, cell_width, cell_height);
        }
    }

    pub fn detect() -> Result<Self, &'static str> {
        let config = Config::new();
        match config {
            Ok(conf) => {
                // let term_size = TermSize::new();
                //
                // let mut font_ratio: f32 = 0.5;
                // let mut cell_width = -1;
                // let mut cell_height = -1;
                //
                // if term_size.width_cells > 0
                //     && term_size.height_cells > 0
                //     && term_size.width_pixels > 0
                //     && term_size.height_pixels > 0
                // {
                //     cell_width = term_size.width_pixels / term_size.width_cells;
                //     cell_height = term_size.height_pixels / term_size.height_cells;
                //     font_ratio = cell_width as f32 / cell_height as f32;
                // }
                // let mut width_cells: i32 = 0;
                // let mut height_cells: i32 = 0;
                //
                // crate::misc::calc_canvas_geometry(src_width, src_height, font_ratio, zoom, stretch)

                conf.set_canvas_mode(conf.term_info.get_best_canvas_mode());
                conf.set_pixel_mode(conf.term_info.get_best_pixel_mode());
                conf.set_passthrough(
                    conf.term_info
                        .get_is_pixel_passthrough_needed(conf.term_info.get_best_pixel_mode()),
                );
                Ok(conf)
            }
            Err(e) => Err(e),
        }
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ffi::chafa_canvas_config_unref(self.raw);
            }
        }
    }
}

struct TermSize {
    width_cells: i32,
    height_cells: i32,
    width_pixels: i32,
    height_pixels: i32,
}
impl TermSize {
    fn new() -> Self {
        let mut width_cells: i32 = -1;
        let mut height_cells: i32 = -1;
        let mut width_pixels: i32 = -1;
        let mut height_pixels: i32 = -1;

        let w_size = terminal::window_size();
        if let Ok(size) = w_size {
            width_cells = size.columns as i32;
            height_cells = size.rows as i32;

            if size.width > 0 {
                width_pixels = size.width as i32;
            }
            if size.height > 0 {
                height_pixels = size.height as i32;
            }
        }

        Self {
            width_cells,
            height_cells,
            width_pixels,
            height_pixels,
        }
    }
}

#[repr(u32)]
pub enum PixelMode {
    /// Pixel data is approximated using character symbols ("ANSI art").
    Symbols = ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_SYMBOLS,
    /// Pixel data is encoded as sixels.
    Sixels = ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_SIXELS,
    /// Pixel data is encoded using the Kitty terminal protocol.
    Kitty = ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_KITTY,
    /// Pixel data is encoded using the iTerm2 terminal protocol.
    Iterm2 = ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_ITERM2,

    /// Last supported pixel mode plus one.
    Max = ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_MAX,
}
impl From<u32> for PixelMode {
    fn from(value: u32) -> Self {
        match value {
            ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_SYMBOLS => PixelMode::Symbols,
            ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_SIXELS => PixelMode::Sixels,
            ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_KITTY => PixelMode::Kitty,
            ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_ITERM2 => PixelMode::Iterm2,
            ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_MAX => PixelMode::Max,
            _ => PixelMode::Symbols,
        }
    }
}
impl From<PixelMode> for u32 {
    fn from(value: PixelMode) -> Self {
        match value {
            PixelMode::Symbols => ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_SYMBOLS,
            PixelMode::Sixels => ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_SIXELS,
            PixelMode::Kitty => ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_KITTY,
            PixelMode::Iterm2 => ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_ITERM2,
            PixelMode::Max => ffi::ChafaPixelMode_CHAFA_PIXEL_MODE_MAX,
        }
    }
}

#[repr(u32)]
pub enum CanvasMode {
    /// Truecolor.
    TrueColor = ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_TRUECOLOR,
    /// 256 colors.
    Indexed256 = ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_256,
    /// 256 colors, but avoid using the lower 16 whose values vary between terminal environments.
    Indexed240 = ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_240,
    /// 16 colors using the aixterm ANSI extension.
    Indexed16 = ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_16,
    /// Default foreground and background colors, plus inversion.
    FgbgBgfg = ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_FGBG_BGFG,
    /// Default foreground and background colors. No ANSI codes will be used.
    FgBg = ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_FGBG,
    /// 8 colors, compatible with original ANSI X3.64.
    Indexed8 = ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_8,
    /// 16 FG colors (8 of which enabled with bold/bright) and 8 BG colors.
    Indexed168 = ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_16_8,

    /// Last supported canvas mode plus one.
    Max = ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_MAX,
}
impl From<u32> for CanvasMode {
    fn from(value: u32) -> Self {
        match value {
            ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_TRUECOLOR => CanvasMode::TrueColor,
            ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_256 => CanvasMode::Indexed256,
            ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_240 => CanvasMode::Indexed240,
            ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_16 => CanvasMode::Indexed16,
            ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_FGBG_BGFG => CanvasMode::FgbgBgfg,
            ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_FGBG => CanvasMode::FgBg,
            ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_8 => CanvasMode::Indexed8,
            ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_16_8 => CanvasMode::Indexed168,
            ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_MAX => CanvasMode::Max,
            _ => CanvasMode::TrueColor,
        }
    }
}
impl From<CanvasMode> for u32 {
    fn from(value: CanvasMode) -> Self {
        match value {
            CanvasMode::TrueColor => ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_TRUECOLOR,
            CanvasMode::Indexed256 => ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_256,
            CanvasMode::Indexed240 => ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_240,
            CanvasMode::Indexed16 => ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_16,
            CanvasMode::FgbgBgfg => ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_FGBG_BGFG,
            CanvasMode::FgBg => ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_FGBG,
            CanvasMode::Indexed8 => ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_8,
            CanvasMode::Indexed168 => ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_INDEXED_16_8,
            CanvasMode::Max => ffi::ChafaCanvasMode_CHAFA_CANVAS_MODE_MAX,
        }
    }
}

#[repr(u32)]
pub enum Passthrough {
    /// No passthrough guards will be used.
    None = ffi::ChafaPassthrough_CHAFA_PASSTHROUGH_NONE,
    /// Passthrough guards for GNU Screen will be used.
    Screen = ffi::ChafaPassthrough_CHAFA_PASSTHROUGH_SCREEN,
    /// Passthrough guards for tmux will be used.
    Tmux = ffi::ChafaPassthrough_CHAFA_PASSTHROUGH_TMUX,

    /// Last supported passthrough mode plus one.
    Max = ffi::ChafaPassthrough_CHAFA_PASSTHROUGH_MAX,
}
impl From<u32> for Passthrough {
    fn from(value: u32) -> Self {
        match value {
            ffi::ChafaPassthrough_CHAFA_PASSTHROUGH_NONE => Passthrough::None,
            ffi::ChafaPassthrough_CHAFA_PASSTHROUGH_SCREEN => Passthrough::Screen,
            ffi::ChafaPassthrough_CHAFA_PASSTHROUGH_TMUX => Passthrough::Tmux,
            ffi::ChafaPassthrough_CHAFA_PASSTHROUGH_MAX => Passthrough::Max,
            _ => Passthrough::None,
        }
    }
}
impl From<Passthrough> for u32 {
    fn from(value: Passthrough) -> Self {
        match value {
            Passthrough::None => ffi::ChafaPassthrough_CHAFA_PASSTHROUGH_NONE,
            Passthrough::Screen => ffi::ChafaPassthrough_CHAFA_PASSTHROUGH_SCREEN,
            Passthrough::Tmux => ffi::ChafaPassthrough_CHAFA_PASSTHROUGH_TMUX,
            Passthrough::Max => ffi::ChafaPassthrough_CHAFA_PASSTHROUGH_MAX,
        }
    }
}
