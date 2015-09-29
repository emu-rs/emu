// TODO: Consider adding error type as well so drivers can report errors properly instead of just panicking

pub type RenderCallback = FnMut(&mut[f32], usize);

pub trait AudioDriver {
    fn set_render_callback(&mut self, callback: Option<Box<RenderCallback>>);

    fn set_is_enabled(&mut self, is_enabled: bool);
    fn is_enabled(&self) -> bool;

    // TODO: set_latency
    // TODO: latency

    fn set_sample_rate(&mut self, sample_rate: i32);
    fn sample_rate(&self) -> i32;
}
