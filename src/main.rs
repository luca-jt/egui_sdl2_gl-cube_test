// uncomment this to disable console window
//#![windows_subsystem = "windows"]

use egui_sdl2_gl::egui::FullOutput;
use egui_sdl2_gl::{egui, sdl2};
use egui_sdl2_gl::egui::*;
use egui_sdl2_gl::egui::load::SizedTexture;
use egui_sdl2_gl::{DpiScaling, ShaderVersion};
use egui_sdl2_gl::sdl2::audio::{AudioSpecDesired, AudioSpecWAV, AudioCVT};
use std::time::Instant;

pub mod render_util;
use crate::render_util::*;
pub mod constants;
use crate::constants::*;
pub mod cpp_bindings;
use crate::cpp_bindings::*;
pub mod audio_util;
use crate::audio_util::*;


fn main()
{
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();
    set_gl_attrs(&video_subsystem);
    
    let window = video_subsystem
        .window("TPG Demo", SCREEN_WIDTH, SCREEN_HEIGHT)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    let _ctx = window.gl_create_context().unwrap();
    enable_vsync(&window);

    // On linux use GLES SL 100+, like so: ShaderVersion::Adaptive
    let (mut painter, mut egui_state) = egui_sdl2_gl::with_sdl2(&window, ShaderVersion::Default, DpiScaling::Default);
    let mut event_pump = sdl2_context.event_pump().unwrap();

    let egui_ctx = egui::Context::default();
    egui_ctx.set_visuals(Visuals::dark());
    
    let mut srgba_buffer: Vec<Color32> = vec![Color32::WHITE; PIC_WIDTH * PIC_HEIGHT];
    // user texture allows mixing egui and gl rendering contexts
    let chip8_tex_id = painter.new_user_texture((PIC_WIDTH as usize, PIC_HEIGHT as usize), &srgba_buffer, false);

    // audio test (not with mixer) (currently works only once)
    let audio_subsystem = sdl2_context.audio().unwrap();
    let audio_spec = AudioSpecDesired{ freq: Some(44100), channels: Some(2), samples: None };
    let audio_device = audio_subsystem
        .open_playback(None, &audio_spec, |spec| {

            let wav = AudioSpecWAV::load_wav("D:/Media/coding/egui_sdl2_test/src/res/StarWars3.wav").unwrap();

            let acvt = AudioCVT::new(
                wav.format,
                wav.channels,
                wav.freq,
                spec.format,
                spec.channels,
                spec.freq
            ).unwrap();

            CopiedAudioData {
                data: acvt.convert(wav.buffer().to_vec()),
                pos: 0,
                volume: 0.25
            }
        }).unwrap();

    let mut circle_radius: f32 = 50.0;
    let start_time = Instant::now();

    'running: loop
    {
        clear_gl_screen();
        srgba_buffer.fill(Color32::WHITE);

        // draw to the image
        draw_circle(circle_radius as usize, PIC_WIDTH / 2, PIC_HEIGHT / 2, Color32::GREEN, &mut srgba_buffer);
        //...
        
        painter.update_user_texture_data(chip8_tex_id, &srgba_buffer);

        egui_state.input.time = Some(start_time.elapsed().as_secs_f64());
        egui_ctx.begin_frame(egui_state.input.take());


        SidePanel::right("right_render")
            .exact_width(SCREEN_WIDTH as f32 * 2.0 / 3.0)
            .show_separator_line(true)
            .resizable(false)
            .show(&egui_ctx, |ui: &mut Ui| {
                ui.vertical_centered(|ui: &mut Ui| {

                    ui.add_space(50.0);

                    let image = Image::new(SizedTexture::new(chip8_tex_id, vec2(PIC_WIDTH as f32, PIC_HEIGHT as f32)));
                    ui.add(image);

                });
            });


        TopBottomPanel::top("left_ui_button")
            .show_separator_line(true)
            .show(&egui_ctx, |ui: &mut Ui| {
                ui.vertical_centered(|ui: &mut Ui| {

                    ui.add_space(50.0);
                    ui.label(RichText::new("Test CMake Link")
                        .font(FontId::new(24.0, FontFamily::Proportional))
                        .extra_letter_spacing(2.0)
                        .color(Color32::from_rgb(255, 255, 255))
                    );
                    ui.add_space(50.0);

                    let button = Button::new(RichText::new("call C++ function")
                            .strong()
                            .font(FontId::new(18.0, FontFamily::Monospace))
                            .color(Color32::from_rgb(0, 0, 0)))
                        .fill(Color32::from_rgb(0, 255, 255))
                        .rounding(Rounding::same(10.0));

                    ui.style_mut().spacing.button_padding = Vec2::new(30.0, 30.0);      
                    if ui.add(button).clicked()
                    {
                        audio_device.resume();
                        unsafe
                        {
                            println!("test i32: {} | test char: {}", test_func().to_string(), test_char() as char);
                        }
                    }
                    ui.add_space(50.0);

                });
            });

        
        CentralPanel::default()
            .show(&egui_ctx, |ui: &mut Ui| {
                ui.vertical_centered(|ui: &mut Ui| {

                    ui.add_space(50.0);

                    let slider = Slider::new(&mut circle_radius, 0.1..=200.0)
                        .text(RichText::new("change radius")
                            .strong()
                            .font(FontId::new(16.0, FontFamily::Proportional))
                            .color(Color32::from_rgb(255, 255, 255)))
                        .fixed_decimals(1);

                    ui.style_mut().spacing.slider_width = 200.0;
                    ui.style_mut().spacing.slider_rail_height = 10.0;
                    ui.add(slider);

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
