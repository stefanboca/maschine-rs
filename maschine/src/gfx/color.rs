#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BlendMode {
    Normal,
    Invert,
    Transparent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    m: u8,
    pub blend_mode: BlendMode,
}

impl Default for Color {
    fn default() -> Self {
        Color {
            r: 0,
            g: 0,
            b: 0,
            m: 0,
            blend_mode: BlendMode::Transparent,
        }
    }
}

impl Color {
    pub const WHITE: Color = Color::from_mono(0xFF);
    pub const BLACK: Color = Color::from_mono(0x00);

    pub const fn from_mono(mono: u8) -> Self {
        Color {
            r: mono,
            g: mono,
            b: mono,
            m: mono,
            blend_mode: BlendMode::Normal,
        }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color {
            r,
            g,
            b,
            m: u8::max(u8::max(r, g), b),
            blend_mode: BlendMode::Normal,
        }
    }

    pub const fn from_rgbm(r: u8, g: u8, b: u8, m: u8) -> Self {
        Color {
            r,
            g,
            b,
            m,
            blend_mode: BlendMode::Normal,
        }
    }

    pub const fn from_blend_mode(blend_mode: BlendMode) -> Self {
        Color {
            r: 0,
            g: 0,
            b: 0,
            m: 0,
            blend_mode,
        }
    }

    pub fn distance(&self, other: &Color) -> f64 {
        f64::sqrt(
            f64::powf((self.red() as f64 - other.red() as f64) * 0.299, 2.0)
                + f64::powf((self.green() as f64 - other.green() as f64) * 0.587, 2.0)
                + f64::powf((self.blue() as f64 - other.blue() as f64) * 0.114, 2.0),
        )
    }

    pub fn red(&self) -> u8 {
        if self.blend_mode == BlendMode::Invert {
            !self.r
        } else {
            self.r
        }
    }

    pub fn green(&self) -> u8 {
        if self.blend_mode == BlendMode::Invert {
            !self.g
        } else {
            self.g
        }
    }

    pub fn blue(&self) -> u8 {
        if self.blend_mode == BlendMode::Invert {
            !self.b
        } else {
            self.b
        }
    }

    pub fn mono(&self) -> u8 {
        if self.blend_mode == BlendMode::Invert {
            !self.m
        } else {
            self.m
        }
    }

    pub fn is_active(&self) -> bool {
        self.mono() > 127
    }

    pub fn as_array_rgb(&self) -> [u8; 3] {
        [self.red(), self.green(), self.blue()]
    }

    pub fn as_array_rgbm(&self) -> [u8; 4] {
        [self.red(), self.green(), self.blue(), self.mono()]
    }
}

impl From<[u8; 3]> for Color {
    fn from(value: [u8; 3]) -> Self {
        Color::from_rgb(value[0], value[1], value[2])
    }
}

impl From<[u8; 4]> for Color {
    fn from(value: [u8; 4]) -> Self {
        Color::from_rgbm(value[0], value[1], value[2], value[3])
    }
}
