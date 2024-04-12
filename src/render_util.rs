use egui_sdl2_gl::sdl2::video::GLProfile;
use egui_sdl2_gl::sdl2::video::SwapInterval;
use egui_sdl2_gl::sdl2::video::Window;
use egui_sdl2_gl::egui::Color32;
use egui_sdl2_gl::sdl2::VideoSubsystem;
use egui_sdl2_gl::gl;
use crate::constants::*;


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


pub fn draw_circle(radius: usize, x_pos: usize, y_pos: usize, srgba_buffer: &mut Vec<Color32>)
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
                srgba_buffer[buffer_index] = Color32::RED;
            }
        }
    }
}


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


pub fn clear_gl_screen()
{
    unsafe
    {
        // Clear the screen to green
        gl::ClearColor(0.3, 0.6, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}
