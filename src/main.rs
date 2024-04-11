pub mod gui;
pub use crate::gui::*;

use eframe::{NativeOptions, CreationContext, run_native, Error, Result};


fn main() -> Result<(), Error>
{
    let sdl2_context = sdl2::init()
        .unwrap();
    let video_subsystem = sdl2_context
        .video()
        .unwrap();
    let window = video_subsystem.window("SDL2 Window", 400, 400)
        .build()
        .unwrap();
    let canvas_builder = window.into_canvas()
        .accelerated()
        .target_texture();

    run_native("TPG Test App",
                NativeOptions::default(),
                Box::new(move |cc: &CreationContext<'_>| {
                    Box::new(TPGApp::new(cc, canvas_builder))
                })
    )
}
