use std::fmt;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn mono(self) -> Color {
        if self.r / 3 + self.g / 3 + self.b / 3 >= 128 {
            WHITE
        } else {
            BLACK
        }
    }

    pub fn inv(self) -> Color {
        Color {
            r: 255 - self.r,
            g: 255 - self.g,
            b: 255 - self.b,
        }
    }

    pub fn hex(self) -> String {
        format!(
            "{:x}{:x}{:x}",
            self.r,
            self.g,
            self.b,
        )
    }
}

#[derive(Clone, Copy)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub bold: bool,
}

impl Default for Style {
    fn default() -> Style {
        Style {
            fg: BLACK,
            bg: WHITE,
            bold: false,
        }
    }
}

impl Style {
    pub fn ansi(self) -> String {
        format!(
            "\x1b[{b};38;2;{fgr};{fgg};{fgb};48;2;{bgr};{bgg};{bgb}m",
            b = if self.bold { 1 } else { 0 },
            fgr = self.fg.r,
            fgg = self.fg.g,
            fgb = self.fg.b,
            bgr = self.bg.r,
            bgg = self.bg.g,
            bgb = self.bg.b,
        )
    }

    pub fn html_style(self) -> String {
        format!(
            "font-weight:{};color:{};background-color:{};",
            if self.bold {
                "bold"
            } else {
                "normal"
            },
            self.fg.hex(),
            self.bg.hex(),
        )
    }
}

pub static RED: Color = Color {
    r: 211,
    g: 47,
    b: 47,
};
pub static PINK: Color = Color {
    r: 194,
    g: 24,
    b: 91,
};
pub static PURPLE: Color = Color {
    r: 123,
    g: 31,
    b: 162,
};
pub static DEEP_PURPLE: Color = Color {
    r: 81,
    g: 45,
    b: 168,
};
pub static INDIGO: Color = Color {
    r: 48,
    g: 63,
    b: 159,
};
pub static BLUE: Color = Color {
    r: 25,
    g: 118,
    b: 210,
};
pub static LIGHT_BLUE: Color = Color {
    r: 2,
    g: 136,
    b: 209,
};
pub static CYAN: Color = Color {
    r: 0,
    g: 151,
    b: 167,
};
pub static TEAL: Color = Color {
    r: 0,
    g: 121,
    b: 107,
};
pub static GREEN: Color = Color {
    r: 56,
    g: 142,
    b: 60,
};
pub static LIGHT_GREEN: Color = Color {
    r: 104,
    g: 159,
    b: 56,
};
pub static LIME: Color = Color {
    r: 175,
    g: 180,
    b: 43,
};
pub static YELLOW: Color = Color {
    r: 251,
    g: 192,
    b: 45,
};
pub static AMBER: Color = Color {
    r: 255,
    g: 160,
    b: 0,
};
pub static ORANGE: Color = Color {
    r: 245,
    g: 124,
    b: 0,
};
pub static DEEP_ORANGE: Color = Color {
    r: 230,
    g: 74,
    b: 25,
};
pub static BROWN: Color = Color {
    r: 93,
    g: 64,
    b: 55,
};
pub static GREY: Color = Color {
    r: 97,
    g: 97,
    b: 97,
};
pub static BLUE_GREY: Color = Color {
    r: 69,
    g: 90,
    b: 100,
};
pub static WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
};
pub static BLACK: Color = Color { r: 0, g: 0, b: 0 };

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:x}{:x}{:x}", self.r, self.g, self.b)
    }
}

pub fn player_colors() -> Vec<Color> {
    vec![
        GREEN,
        RED,
        BLUE,
        AMBER,
        PURPLE,
        BROWN,
        BLUE_GREY,
    ]
}
