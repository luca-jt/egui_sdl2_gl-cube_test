#[link(name="test", kind="static")]

extern "C"
{
    fn test_func() -> i32;
}


use egui_backend::egui::FullOutput;
use egui_backend::sdl2::video::GLProfile;
use egui_backend::{egui, gl, sdl2};
use egui_backend::egui::*;
use egui_backend::{sdl2::event::Event, DpiScaling, ShaderVersion};
use std::time::Instant;
use sdl2::video::SwapInterval;

use egui_sdl2_gl as egui_backend;

const SCREEN_WIDTH: u32 = 600;
const SCREEN_HEIGHT: u32 = 600;


fn main()
{
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    // On linux, OpenGL ES Mesa driver 22.0.0+ can be used like so:
    //gl_attr.set_context_profile(GLProfile::GLES);
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_double_buffer(true);
    gl_attr.set_multisample_samples(4);

    let window = video_subsystem
        .window("SDL2 Window", SCREEN_WIDTH, SCREEN_HEIGHT)
        .opengl()
        .build()
        .unwrap();

    let _ctx = window.gl_create_context().unwrap();

    // On linux use GLES SL 100+, like so:
    //let shader_ver = ShaderVersion::Adaptive;
    let shader_ver = ShaderVersion::Default;

    let (mut painter, mut egui_state) = egui_backend::with_sdl2(&window, shader_ver, DpiScaling::Default);
    let egui_ctx = egui::Context::default();
    let mut event_pump = sdl2_context.event_pump().unwrap();

    if let Err(error) = window.subsystem().gl_set_swap_interval(SwapInterval::VSync)
    {
        println!("Failed to gl_set_swap_interval(SwapInterval::VSync): {}", error);
    }
    else if let Err(error) = window
        .subsystem()
        .gl_set_swap_interval(SwapInterval::Immediate)
    {
        println!("Failed to gl_set_swap_interval(SwapInterval::Immediate): {}", error);
    }

    let start_time = Instant::now();

    'running: loop
    {
        unsafe
        {
            // Clear the screen to green
            gl::ClearColor(0.3, 0.6, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        egui_state.input.time = Some(start_time.elapsed().as_secs_f64());
        egui_ctx.begin_frame(egui_state.input.take());

        CentralPanel::default().show(&egui_ctx, |ui| {

            ui.vertical_centered(|ui: &mut Ui| {

                ui.add_space(10.0);
                ui.label(RichText::new("Test CMake Link")
                    .font(FontId::new(20.0, FontFamily::Proportional))
                    .underline()
                );
                ui.add_space(20.0);

                let button = Button::new(RichText::new("call C++ function")
                        .strong()
                        .font(FontId::new(10.0, FontFamily::Monospace))
                        .color(Color32::from_rgb(0, 0, 0)))
                    .fill(Color32::from_rgb(0, 255, 255))
                    .rounding(Rounding::same(50.0))
                    .min_size(vec2(100.0, 100.0));
                                
                if ui.add(button).clicked()
                {
                    let test_int;
                    unsafe { test_int = test_func(); }
                    println!("{}", test_int.to_string());
                }

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                //ui.image("source");

            });
        });

        // render test texture to gui
        //...

        let FullOutput
        {
            platform_output,
            textures_delta,
            shapes,
            pixels_per_point,
            viewport_output,
        } = egui_ctx.end_frame();

        // Process ouput
        egui_state.process_output(&window, &platform_output);

        let paint_jobs = egui_ctx.tessellate(shapes, pixels_per_point);
        painter.paint_jobs(None, textures_delta, paint_jobs);
        window.gl_swap_window();

        let repaint_after = viewport_output
            .get(&egui::ViewportId::ROOT)
            .expect("Missing ViewportId::ROOT")
            .repaint_delay;

        if !repaint_after.is_zero()
        {
            if let Some(event) = event_pump.wait_event_timeout(5)
            {
                match event {
                    Event::Quit { .. } => break 'running,
                    _ => {
                        egui_state.process_input(&window, event, &mut painter);
                    }
                }
            }
        }
        else
        {
            for event in event_pump.poll_iter()
            {
                match event {
                    Event::Quit { .. } => break 'running,
                    _ => {
                        egui_state.process_input(&window, event, &mut painter);
                    }
                }
            }
        }
    }
}
