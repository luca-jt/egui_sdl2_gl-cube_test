pub mod gui;
pub use crate::gui::*;

use eframe::{NativeOptions, CreationContext, run_native, Error, Result};
//use sdl2::{pixels::Color, event::Event};


fn main() -> Result<(), Error>
{
    run_native("TPG Test App",
                NativeOptions::default(),
                Box::new(|_cc: &CreationContext<'_>| {
                    Box::new(TPGApp {})
                })
    )
}
