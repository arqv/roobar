use crate::color::Color;
use crate::Error;
use std::sync::*;

pub trait Component {
    fn get_color(&self) -> Color;
    fn show(&self) -> Option<String>;
    fn update(&self) -> Result<(), Error> {
        Ok(())
    }
}

pub struct ComponentList(pub Vec<Arc<dyn Component>>);
impl ComponentList {
    pub fn new() -> ComponentList {
        ComponentList(vec![])
    }
    pub fn push(&mut self, component: Arc<dyn Component>) {
        self.0.push(component);
    }
    pub fn show_all_sync(
        &self,
        separator: &'static str,
        fmt: &impl crate::formatter::Formatter,
        color: Color,
    ) -> String {
        let separator = fmt.colorize(String::from(separator), color);
        self.0
            .iter()
            .map(|c| c.show().unwrap_or_else(|| String::from("n/a")))
            .collect::<Vec<String>>()
            .join(&separator)
    }
}
