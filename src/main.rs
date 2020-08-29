extern crate nom;

mod color;
mod component;
mod components;
mod formatter;

use std::sync::{Arc, RwLock};
use std::path::Path;

use color::{Color, RGB};
use formatter::DzenFormatter;

#[derive(Debug, Copy, Clone)]
pub enum ErrorKind {
    Unknown,
    Guard
}

#[derive(Debug, Copy, Clone)]
pub struct Error {
    kind: ErrorKind,
    payload: Option<&'static str>,
}

// Can I sleep please? It's 00:16.
const FMT: DzenFormatter = DzenFormatter {};

pub fn main() -> Result<(), Error> {
    let mut components = component::ComponentList::new();
    components.push(Arc::new(RwLock::new(components::Text {
        color: Color::new(None, RGB(245, 45, 12)),
        text: String::from("Test"),
        fmt: &FMT,
    })));
    components.push(Arc::new(RwLock::new(components::Battery::new(
        Color::new(None, RGB(0,255,0)),
        Color::new(None, RGB(255,0,0)),
        Color::new(None, RGB(255,255,0)),
        Path::new("/sys/class/power_supply/BAT1"),
        Path::new("/sys/class/power_supply/ACAD"),
        0.15,
        &FMT
    ))));

    components.update_all_sync()?;

    println![
        "{}",
        components.show_all_sync(" | ", &FMT, Color::new(None, RGB(127, 127, 127)))?
    ];

    Ok(())
}
