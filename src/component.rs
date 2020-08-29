use crate::color::Color;
use crate::{Error, ErrorKind};
use std::{slice::Iter, sync::{Arc, RwLock}};

pub trait Component {
    fn get_color(&self) -> Color;
    fn show(&self) -> Option<String>;
    fn update(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

pub struct ComponentList(pub Vec<Arc<RwLock<dyn Component>>>);
impl ComponentList {
    pub fn new() -> ComponentList {
        ComponentList(vec![])
    }
    pub fn push(&mut self, component: Arc<RwLock<dyn Component>>) {
        self.0.push(component);
    }
    pub fn show_all_sync(
        &self,
        separator: &'static str,
        fmt: &impl crate::formatter::Formatter,
        color: Color,
    ) -> Result<String, Error> {
        let separator = fmt.colorize(String::from(separator), color);
        self.iter()
            .map(|c| { 
                let tmp = c.read();
                if tmp.is_err() {
                    Err(Error {
                        kind: ErrorKind::Guard,
                        payload: Some("Failed to acquire read guard.")
                    })
                } else {
                    Ok(())
                }
            })
            .collect::<Result<(), Error>>()?;

        Ok(self.iter()
            .map(|c| c.read().unwrap().show().unwrap_or_else(|| String::from("n/a")))
            .collect::<Vec<String>>()
            .join(&separator))
    }
    pub fn update_all_sync(&mut self) -> Result<(), Error> {
        self.iter()
            .map(|c| {
                let tmp = c.write();
                if tmp.is_err() {
                    Err(Error {
                        kind: ErrorKind::Guard,
                        payload: Some("Failed to acquire write guard.")
                    })
                } else {
                    Ok(())
                }
            })
            .collect::<Result<(), Error>>()?;

        self.iter()
            .map(|c| c.write().unwrap().update())
            .collect()
    }
    pub fn iter(&self) -> Iter<Arc<RwLock<dyn Component>>> {
        self.0.iter()
    }
}
