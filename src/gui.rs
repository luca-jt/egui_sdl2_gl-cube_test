#[link(name="test", kind="static")]

extern "C"
{
    fn test_func() -> i32;
}


extern crate sdl2;

use eframe::{Frame, CreationContext};
use egui::*;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;


pub struct TPGApp
{
    canvas: WindowCanvas
}


impl TPGApp
{
    pub fn new(_cc: &CreationContext<'_>, canvas: WindowCanvas) -> TPGApp
    {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        TPGApp { canvas }
    }
}


impl eframe::App for TPGApp
{
    fn update(&mut self, ctx: &Context, _frame: &mut Frame)
    {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {

            ui.vertical_centered(|ui: &mut Ui| {

                ui.add_space(80.0);

                ui.label(RichText::new("Test CMake Link")
                    .font(FontId::new(20.0, FontFamily::Proportional))
                    .underline()
                );

                ui.add_space(80.0);

                let button = Button::new(RichText::new("call C++ function")
                                                .strong()
                                                .font(FontId::new(16.0, FontFamily::Monospace)))
                                    .fill(Color32::from_rgb(0, 255, 255))
                                    .rounding(Rounding::same(100.0))
                                    .min_size(vec2(200.0, 200.0));
                                
                let button_resp: Response = ui.add(button);

                if button_resp.clicked()
                {
                    let test_int;
                    unsafe { test_int = test_func(); }

                    println!("{}", test_int.to_string());
                }


                self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                self.canvas.clear();
                self.canvas.set_draw_color(Color::RGB(255, 0, 0));
                self.canvas.draw_line((10, 10), (400, 400)).ok().unwrap();
                self.canvas.present();

                /* let texture_creator = self.canvas.texture_creator();
                let mut texture = texture_creator
                    .create_texture_target(sdl2::pixels::PixelFormatEnum::RGB888, 800, 600).expect("sdl texture failed");

                self.canvas.with_texture_canvas(&mut texture, |texture_canvas| {
                    texture_canvas.clear();
                    texture_canvas.set_draw_color(Color::RGB(255, 0, 0));
                    texture_canvas.draw_line((100, 100), (200, 200)).ok().unwrap();
                })
                .expect("canvas rendering failed"); */

            });

        });
    }
}
