use std::{
    slice::Iter,
    sync::{Arc, RwLock},
};

use crate::color::Color;
use crate::{Error, ErrorKind};

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
        let mut guards = vec![];
        self.iter()
            .map(|c| {
                if let Ok(guard) = c.read() {
                    guards.push(guard);
                    Ok(())
                } else {
                    Err(Error {
                        kind: ErrorKind::GuardError,
                        payload: Some("failed to acquire read guard"),
                    })
                }
            })
            .collect::<Result<(), Error>>()?;

        Ok(guards
            .iter()
            .map(|c| c.show().unwrap_or_else(|| String::from("n/a")))
            .collect::<Vec<String>>()
            .join(&separator))
    }
    pub fn update_all_sync(&mut self) -> Result<(), Error> {
        let mut guards = vec![];
        self.iter()
            .map(|c| {
                let tmp = c.write();
                if tmp.is_err() {
                    Err(Error {
                        kind: ErrorKind::GuardError,
                        payload: Some("failed to acquire write guard"),
                    })
                } else {
                    if let Ok(tmp) = tmp {
                        guards.push(tmp);
                    }
                    Ok(())
                }
            })
            .collect::<Result<(), Error>>()?;

        guards.iter_mut().map(|c| c.update()).collect()
    }
    pub fn iter(&self) -> Iter<Arc<RwLock<dyn Component>>> {
        self.0.iter()
    }
}
