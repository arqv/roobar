pub trait ColorCode {
    fn repr(&self) -> String;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RGB(pub u8, pub u8, pub u8);
impl RGB {
    pub fn as_hex(&self) -> String {
        format!["#{:02X}{:02X}{:02X}", self.0, self.1, self.2]
    }
}

#[derive(Copy, Clone)]
pub struct Color {
    pub bg: Option<RGB>,
    pub fg: Option<RGB>,
}
impl Default for Color {
    fn default() -> Color {
        Color { bg: None, fg: None }
    }
}
impl Color {
    pub fn new(bg: impl Into<Option<RGB>>, fg: impl Into<Option<RGB>>) -> Color {
        Color {
            bg: bg.into(),
            fg: fg.into(),
        }
    }
}
