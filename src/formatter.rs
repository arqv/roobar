use crate::color::Color;

pub trait Formatter {
    fn get_code(&self, color: Color) -> String;
    fn colorize(&self, string: String, color: Color) -> String;
}

pub struct Terminal {}
impl Formatter for Terminal {
    fn get_code(&self, _color: Color) -> String {
        unimplemented!();
    }
    fn colorize(&self, string: String, _color: Color) -> String {
        string
    }
}

pub struct Dzen {}
impl Formatter for Dzen {
    fn get_code(&self, color: Color) -> String {
        format![
            "^fg({})^bg({})",
            match color.fg {
                Some(color) => color.as_hex(),
                None => String::new(),
            },
            match color.bg {
                Some(color) => color.as_hex(),
                None => String::new(),
            }
        ]
    }

    fn colorize(&self, string: String, color: Color) -> String {
        format![
            "{}{}{}",
            self.get_code(color),
            string,
            self.get_code(Color::default())
        ]
    }
}
