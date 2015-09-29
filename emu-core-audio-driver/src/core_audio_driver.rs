extern crate emu_audio_types;
extern crate coreaudio_rs;

use std::slice;
use self::emu_audio_types::audio_driver::{AudioDriver, RenderCallback};
use self::coreaudio_rs::audio_unit;
use self::coreaudio_rs::audio_unit::{stream_format, audio_format};

pub struct CoreAudioDriver {
    audio_unit: audio_unit::AudioUnit,
    is_enabled: bool // TODO: Remove if we can query this state from the AudioUnit instead
}

impl CoreAudioDriver {
    pub fn new() -> CoreAudioDriver {
        let audio_unit = audio_unit::AudioUnit::new(audio_unit::Type::Output, audio_unit::SubType::DefaultOutput).unwrap();

        audio_unit.set_stream_format(stream_format::StreamFormat {
            sample_rate: 44100.0,
            audio_format: audio_format::AudioFormat::LinearPCM(Some(audio_format::LinearPCMFlag::IsFloat)),
            bytes_per_packet: 2 * 4,
            frames_per_packet: 1,
            bytes_per_frame: 2 * 4,
            channels_per_frame: 2,
            bits_per_channel: 32
        }).ok();

        audio_unit.start().ok();

        CoreAudioDriver {
            audio_unit: audio_unit,
            is_enabled: true
        }
    }
}

impl AudioDriver for CoreAudioDriver {
    fn set_render_callback(&mut self, callback: Option<Box<RenderCallback>>) {
        self.audio_unit.render_callback(match callback {
            Some(mut callback) => Some(Box::new(move |buffers, num_frames| {
                // TODO: This is temporary, as I believe I've uncovered a bug in coreaudio-rs regarding the size of the buffer
                // passed to the rendering callback.
                let buffer = unsafe {
                    let slice_ptr = &mut buffers[0][0] as *mut f32;
                    slice::from_raw_parts_mut(slice_ptr, num_frames * 2)
                };
                callback(buffer, num_frames);
                Ok(())
            })),
            _ => None
        }).ok();
    }

    fn set_is_enabled(&mut self, is_enabled: bool) {
        if is_enabled == self.is_enabled {
            return;
        }

        if is_enabled {
            self.audio_unit.start().ok();
        } else {
            self.audio_unit.stop().ok();
        }

        self.is_enabled = is_enabled;
    }

    fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    fn set_sample_rate(&mut self, sample_rate: i32) {
        self.audio_unit.set_sample_rate(sample_rate as f64).ok();
    }

    fn sample_rate(&self) -> i32 {
        self.audio_unit.sample_rate().unwrap() as i32
    }
}
