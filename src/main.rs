mod color;
mod component;
mod components;
mod formatter;
use color::{Color, RGB};
use component::ComponentList;
use formatter::Dzen;
use std::path::Path;
use std::sync::{Arc, RwLock};

#[derive(Debug, Copy, Clone)]
pub enum ErrorKind {
    Unknown,
    GuardError,
    FileError,
}
#[derive(Debug, Copy, Clone)]
pub struct Error {
    kind: ErrorKind,
    payload: Option<&'static str>,
}

const FMT: Dzen = Dzen {};

macro_rules! add_component {
    ($vec:expr, $component:expr) => {
        $vec.push(Arc::new(RwLock::new($component)));
    };
}

pub fn main() -> Result<(), Error> {
    let mut components = ComponentList::new();
    add_component![
        components,
        components::Battery::new(
            Color::new(None, RGB(0, 255, 0)),
            Color::new(None, RGB(255, 0, 0)),
            Color::new(None, RGB(255, 255, 0)),
            Path::new("/sys/class/power_supply/BAT1"),
            Path::new("/sys/class/power_supply/ACAD"),
            0.15,
            &FMT
        )
    ];
    components.update_all_sync()?;
    println![
        "{}",
        components.show_all_sync(" :: ", &FMT, Color::new(None, RGB(127, 127, 127)))?
    ];
    Ok(())
}
