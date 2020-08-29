mod color;
mod component;
mod components;
mod formatter;

use std::sync::Arc;

use color::{Color, RGB};
use formatter::DzenFormatter;

#[derive(Debug)]
pub enum ErrorKind {
    Unknown,
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    payload: Option<String>,
}

const FMT: DzenFormatter = DzenFormatter {};

pub fn main() -> Result<(), Error> {
    let mut components = component::ComponentList::new();
    components.push(Arc::new(components::Text {
        color: Color::new(None, RGB(245, 45, 12)),
        text: String::from("Hello, World!"),
        fmt: &FMT,
    }));
    components.push(Arc::new(components::Text {
        color: Color::new(None, RGB(24, 233, 62)),
        text: String::from("Another component."),
        fmt: &FMT,
    }));

    println![
        "{}",
        components.show_all_sync("|", &FMT, Color::new(None, RGB(78, 78, 78)))
    ];

    Ok(())
}
