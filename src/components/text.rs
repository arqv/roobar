use crate::color::Color;
use crate::component::Component;
use crate::formatter::Formatter;

pub struct Text {
    pub color: Color,
    pub text: String,
    pub fmt: &'static dyn Formatter,
}
impl Component for Text {
    fn get_color(&self) -> Color {
        self.color
    }
    fn show(&self) -> Option<String> {
        Some(self.fmt.colorize(self.text.clone(), self.get_color()))
    }
}
