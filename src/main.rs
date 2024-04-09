#[link(name="test", kind="static")]

extern "C"
{
    fn test_func() -> i32;
}

use eframe::{Frame, NativeOptions, CreationContext, run_native};
use egui::*;


struct TPGApp {}

impl eframe::App for TPGApp
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame)
    {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {

            ui.vertical_centered(|ui: &mut Ui| {

                ui.add_space(80.0);

                ui.label(RichText::new("Test CMake Link")
                    .font(FontId::new(20.0, FontFamily::Proportional))
                    .underline()
                );

                ui.add_space(80.0);

                let button = egui::Button::new(RichText::new("call C++ function")
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

            });

        });
    }
}


fn main() -> eframe::Result<(), eframe::Error>
{
    run_native("TPG Test App",
                NativeOptions::default(),
                Box::new(|_cc: &CreationContext<'_>| {
                    Box::new(TPGApp {})
                })
    )
}
