pub mod gui;
pub use crate::gui::*;

use eframe::{NativeOptions, CreationContext, run_native, Error, Result};


fn main() -> Result<(), Error>
{
    let sdl2_context = sdl2::init().expect("sdl init failed");
    let video_subsystem = sdl2_context.video().expect("sdl video subs failed");
    let window = video_subsystem.window("SDL2 Window", 800, 600).build().expect("sdl window failed");
    let canvas = window.into_canvas().build().expect("sdl canvas failed");

    run_native("TPG Test App",
                NativeOptions::default(),
                Box::new(move |cc: &CreationContext<'_>| {
                    Box::new(TPGApp::new(cc, canvas))
                })
    )
}
