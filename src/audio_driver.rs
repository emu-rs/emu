pub type RenderCallback = FnMut(&mut[f32], usize);

pub trait AudioDriver {
    // TODO: fn set_render_callback(&mut self, callback: RenderCallback /* TODO: user_data */);

    // TODO: set_enabled
    // TODO: is_enabled

    // TODO: set_latency
    // TODO: latency

    fn set_sample_rate(&mut self, sample_rate: i32);
    fn sample_rate(&self) -> i32;
}
