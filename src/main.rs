use egui_sdl2_gl::egui::FullOutput;
use egui_sdl2_gl::{egui, sdl2};
use egui_sdl2_gl::egui::*;
use egui_sdl2_gl::egui::load::SizedTexture;
use egui_sdl2_gl::{DpiScaling, ShaderVersion};
use std::time::Instant;

pub mod render_util;
use crate::render_util::*;
pub mod constants;
use crate::constants::*;
pub mod cpp_bindings;
use crate::cpp_bindings::*;


fn main()
{
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();
    set_gl_attrs(&video_subsystem);
    
    let window = video_subsystem
        .window("SDL2 Window", SCREEN_WIDTH, SCREEN_HEIGHT)
        .opengl()
        .build()
        .unwrap();

    let _ctx = window.gl_create_context().unwrap();
    enable_vsync(&window);

    // On linux use GLES SL 100+, like so: ShaderVersion::Adaptive
    let (mut painter, mut egui_state) = egui_sdl2_gl::with_sdl2(&window, ShaderVersion::Default, DpiScaling::Default);
    let egui_ctx = egui::Context::default();
    let mut event_pump = sdl2_context.event_pump().unwrap();
    // init the frame buffer
    let mut srgba_buffer: Vec<Color32> = vec![Color32::WHITE; PIC_WIDTH * PIC_HEIGHT];
    // user texture allows mixing egui and gl rendering contexts
    let chip8_tex_id = painter.new_user_texture((PIC_WIDTH as usize, PIC_HEIGHT as usize), &srgba_buffer, false);

    let mut circle_radius: f32 = 50.0;
    let start_time = Instant::now();

    'running: loop
    {
        clear_gl_screen();

        srgba_buffer.fill(Color32::WHITE);
        // draw to the image
        draw_circle(circle_radius as usize, PIC_WIDTH / 2, PIC_HEIGHT / 2, Color32::RED, &mut srgba_buffer);
        //...
        
        painter.update_user_texture_data(chip8_tex_id, &srgba_buffer);

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
                        .font(FontId::new(18.0, FontFamily::Monospace))
                        .color(Color32::from_rgb(0, 0, 0)))
                    .fill(Color32::from_rgb(0, 255, 255))
                    .rounding(Rounding::same(10.0))
                    .min_size(vec2(300.0, 80.0));
                                
                if ui.add(button).clicked()
                {
                    unsafe { println!("{} | {}", test_func().to_string(), test_char() as char); }
                }

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(20.0);

                let slider = Slider::new(&mut circle_radius, 0.1..=100.0)
                    .text("change radius")
                    .fixed_decimals(1);

                ui.add(slider);
                ui.add_space(40.0);

                let image = Image::new(SizedTexture::new(chip8_tex_id, vec2(PIC_WIDTH as f32, PIC_HEIGHT as f32)));
                ui.add(image);

            });
        });

        let FullOutput
        {
            platform_output,
            textures_delta,
            shapes,
            pixels_per_point,
            viewport_output,
        } = egui_ctx.end_frame();

        egui_state.process_output(&window, &platform_output);

        let paint_jobs = egui_ctx.tessellate(shapes, pixels_per_point);
        painter.paint_jobs(None, textures_delta, paint_jobs);

        window.gl_swap_window();

        let repaint_after = viewport_output
            .get(&egui::ViewportId::ROOT)
            .expect("Missing ViewportId::ROOT")
            .repaint_delay;

        if handle_events(&mut event_pump, &repaint_after, &window, &mut egui_state, &mut painter).is_err()
        {
            break 'running;
        }
    }
}
