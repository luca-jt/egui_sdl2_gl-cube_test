extern crate sdl2;

use eframe::{Frame, CreationContext};
use egui::*;
use sdl2::pixels::Color;
use sdl2::render::{CanvasBuilder, WindowCanvas};


pub struct TPGApp
{
    canvas: WindowCanvas,
}


impl TPGApp
{
    pub fn new(_cc: &CreationContext<'_>, canvas_builder: CanvasBuilder) -> TPGApp
    {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        let c = canvas_builder.build().unwrap();

        TPGApp { canvas: c }
    }
}


impl eframe::App for TPGApp
{
    fn update(&mut self, ctx: &Context, _frame: &mut Frame)
    {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {

            ui.vertical_centered(|ui: &mut Ui| {

                ui.add_space(20.0);

                ui.label(RichText::new("Test CMake Link")
                    .font(FontId::new(20.0, FontFamily::Proportional))
                    .underline()
                );

            });

        });


        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.canvas.draw_line((100, 100), (200, 200)).ok().unwrap();
        self.canvas.present();
    }
}
