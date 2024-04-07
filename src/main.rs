#[link(name="test", kind="static")]

extern "C"
{
    fn test_func() -> i32;
}

use eframe::*;
use egui::{CentralPanel, Response, Ui};


struct TPGApp {}

impl eframe::App for TPGApp
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame)
    {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.label("text placeholder");

            let button_resp: Response = ui.button("test button");

            if button_resp.clicked()
            {
                let test_int;
                unsafe { test_int = test_func(); }

                println!("{}", test_int.to_string());
            }
        });
    }
}


fn main() -> eframe::Result<(), eframe::Error>
{
    run_native("TPG Test App", NativeOptions::default(), Box::new(|_cc: &CreationContext<'_>| {
        Box::new(TPGApp {})
    }))
}
