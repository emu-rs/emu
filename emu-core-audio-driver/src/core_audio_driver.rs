extern crate emu_audio_types;
extern crate coreaudio_rs;

use self::emu_audio_types::audio_driver::{AudioDriver, RenderCallback};
use self::coreaudio_rs::audio_unit;

pub struct CoreAudioDriver {
    audio_unit: audio_unit::AudioUnit,
    is_enabled: bool // TODO: Remove
}

impl CoreAudioDriver {
    pub fn new() -> CoreAudioDriver {
        let audio_unit = audio_unit::AudioUnit::new(audio_unit::Type::Output, audio_unit::SubType::DefaultOutput).unwrap();

        // TODO
        /*let sample_rate = 44100;
        let mut stream_desc = au::AudioStreamBasicDescription {
            mSampleRate: sample_rate as f64,
            mFormatID: au::kAudioFormatLinearPCM,
            mFormatFlags: au::kAudioFormatFlagIsFloat as u32,
            mFramesPerPacket: 1,
            mChannelsPerFrame: 2,
            mBitsPerChannel: 32,
            mBytesPerPacket: 2 * 4,
            mBytesPerFrame: 2 * 4,
            mReserved: 0
        };
        check_os_error!(
            au::AudioUnitSetProperty(
                instance,
                au::kAudioUnitProperty_StreamFormat,
                au::kAudioUnitScope_Input,
                0,
                &mut stream_desc as *mut _ as *mut libc::c_void,
                mem::size_of::<au::AudioStreamBasicDescription>() as u32),
                CoreAudioDriverError::AudioUnitSetPropertyFailed);*/

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
            Some(mut callback) => Some(Box::new(move |buffer, num_frames| {
                // TODO: lol :D
                callback(buffer[0], num_frames / 2);
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

// TODO
/*extern "C" fn render_proc(
    in_ref_con: *mut libc::c_void,
    _: *mut au::AudioUnitRenderActionFlags,
    _: *const au::AudioTimeStamp,
    _: au::UInt32,
    in_number_frames: au::UInt32,
    io_data: *mut au::AudioBufferList) -> au::OSStatus {
    let callback: *mut Box<RenderCallback> = in_ref_con as *mut _;
    unsafe {
        let slice_ptr = (*io_data).mBuffers[0].mData as *mut libc::c_float;
        let buffer = slice::from_raw_parts_mut(slice_ptr, (in_number_frames * 2) as usize);

        (*callback)(buffer, in_number_frames as usize);
    }

    0
}*/
