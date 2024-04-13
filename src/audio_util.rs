use egui_sdl2_gl::sdl2::audio::AudioCallback;


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
