use egui_sdl2_gl::sdl2::audio::AudioCallback;
use egui_sdl2_gl::sdl2::mixer;
use std::thread::sleep;
use std::time::Duration;


pub struct CopiedAudioData
{
    pub data: Vec<u8>,
    pub pos: usize,
    pub volume: f32
}

impl AudioCallback for CopiedAudioData
{
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8])
    {
        for dst in out.iter_mut()
        {
            let pre_scale = *self.data.get(self.pos).unwrap_or(&128);
            let scaled_signed_float = (pre_scale as f32 - 128.0) * self.volume;
            let scaled = (scaled_signed_float + 128.0) as u8;
            *dst = scaled;
            self.pos += 1;
        }
    }
}


/// play sound effect from given file path
pub fn play_sfx(file_path: &str) // TODO: do this in seperate thread
{
    let mut sfx = mixer::Chunk::from_file(file_path).unwrap();
    sfx.set_volume(25);
    mixer::Channel::all()
        .play(&sfx, 1)
        .unwrap();
    sleep(Duration::from_secs(1));
}
