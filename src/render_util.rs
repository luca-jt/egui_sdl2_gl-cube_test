use std::time::Duration;
use egui_sdl2_gl::painter::Painter;
use egui_sdl2_gl::sdl2::video::GLProfile;
use egui_sdl2_gl::sdl2::video::SwapInterval;
use egui_sdl2_gl::sdl2::video::Window;
use egui_sdl2_gl::egui::Color32;
use egui_sdl2_gl::sdl2::EventPump;
use egui_sdl2_gl::sdl2::VideoSubsystem;
use egui_sdl2_gl::sdl2::event::Event;
use egui_sdl2_gl::gl;
use egui_sdl2_gl::EguiStateHandler;
use crate::constants::*;


/// enables vsync for given SDL window
pub fn enable_vsync(window: &Window)
{
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
}


/// draw a filled circle with given radius at (x,y) with given color to the buffer
pub fn draw_circle(radius: usize, x_pos: usize, y_pos: usize, color: Color32, srgba_buffer: &mut Vec<Color32>)
{
    for y in 0..PIC_HEIGHT
    {
        for x in 0..PIC_WIDTH
        {
            let x_with_offset = x as i64 - x_pos as i64;
            let y_with_offset = y as i64 - y_pos as i64;
            let buffer_index: usize = x + y * PIC_WIDTH;

            if x_with_offset * x_with_offset + y_with_offset * y_with_offset <= (radius * radius) as i64
            {
                srgba_buffer[buffer_index] = color;
            }
        }
    }
}


/// set the gl attributes for the video_subsystem
pub fn set_gl_attrs(video_subsystem: &VideoSubsystem)
{
    let gl_attr = video_subsystem.gl_attr();
    // On linux, OpenGL ES Mesa driver 22.0.0+ can be used like so:
    //gl_attr.set_context_profile(GLProfile::GLES);
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_double_buffer(true);
    gl_attr.set_multisample_samples(4);
    gl_attr.set_framebuffer_srgb_compatible(true);
    gl_attr.set_context_version(3, 2);
}


/// clears the gl screen with color white
pub fn clear_gl_screen()
{
    unsafe
    {
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}


/// handle the SDL events with EventPump ep
pub fn handle_events(ep: &mut EventPump, ra: &Duration, w: &Window, es: &mut EguiStateHandler, p: &mut Painter) -> Result<(), ()>
{
    if !ra.is_zero()
    {
        if let Some(event) = ep.wait_event_timeout(5)
        {
            match event
            {
                Event::Quit { .. } => return Err(()),
                _ => {
                    es.process_input(&w, event, p);
                }
            }
        }
    }
    else
    {
        for event in ep.poll_iter()
        {
            match event
            {
                Event::Quit { .. } => return Err(()),
                //... other events go here
                _ => {
                    es.process_input(&w, event, p);
                }
            }
        }
    }

    Ok(())
}
