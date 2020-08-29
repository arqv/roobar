use crate::{
    component::Component,
    color::Color,
    formatter::Formatter,
    Error, ErrorKind
};

use std::path::Path;

pub struct Battery {
    pub colors: (Color, Color, Color),
    pub batpath: &'static Path,
    pub acpath: &'static Path,
    pub threshold: f64,
    pub fmt: &'static dyn Formatter,
    batcap: f64,
    acconn: bool,
    _res: Result<(), Error>
}

impl Battery {
    pub fn new(
        batt: Color,
        lowbatt: Color,
        ac: Color,
        batpath: &'static Path,
        acpath: &'static Path,
        threshold: f64,
        fmt: &'static dyn Formatter
    ) -> Battery {
        Battery {
            colors: (batt, lowbatt, ac),
            batpath, acpath, threshold, fmt,
            acconn: false, batcap: 0.0,
            _res: Ok(())
        }
    }
}

impl Component for Battery {
    fn get_color(&self) -> Color {
        if self.acconn {
            self.colors.2
        } else {
            if self.batcap <= self.threshold {
                self.colors.1
            } else {
                self.colors.0
            }
        }
    }
    fn update(&mut self) -> Result<(), Error> {
        self.batcap = 0.69;
        self.acconn = false;
        self._res = Ok(());
        self._res
    }
    fn show(&self) -> Option<String> {
        if self._res.is_err() {
            None
        } else {
            if self.acconn {
                Some(format![
                    "{}: {}%",
                    self.fmt.colorize(String::from("AC"), self.get_color()),
                    (self.batcap * 100.0) as i8
                ])
            } else {
                Some(format![
                    "{}: {}%",
                    self.fmt.colorize(String::from("Battery"), self.get_color()),
                    (self.batcap * 100.0) as i8
                ])
            }
        }
    }
}