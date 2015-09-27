extern crate emu_audio_types;
extern crate coreaudio_sys as bindings;
extern crate libc;

use std::ptr;
use std::mem;
use std::slice;
use self::emu_audio_types::audio_driver::{AudioDriver, RenderCallback};
use self::bindings::audio_unit as au;

const COMPONENT_TYPE_OUTPUT: libc::c_uint = 0x61756f75;
const COMPONENT_SUB_TYPE_DEFAULT_OUTPUT: libc::c_uint = 0x64656620;

#[derive(Debug)]
pub enum CoreAudioDriverError {
    AudioComponentNotFound,
    AudioComponentInstanceCreationFailed,
    AudioComponentInstanceInitializationFailed,
    AudioUnitSetPropertyFailed,
    AudioUnitSetRenderCallbackFailed,
    AudioOutputUnitStartFailed
}

pub struct CoreAudioDriver {
    instance: au::AudioComponentInstance,
    callback: *mut libc::c_void,
    is_enabled: bool
}

macro_rules! check_os_error {
    ($expr:expr,$err:expr) => ({
        if $expr != 0 {
            return Err($err);
        }
    })
}

impl CoreAudioDriver {
    pub fn new(callback: Box<RenderCallback>) -> Result<CoreAudioDriver, CoreAudioDriverError> {
        let desc = au::AudioComponentDescription {
            componentType: COMPONENT_TYPE_OUTPUT,
            componentSubType: COMPONENT_SUB_TYPE_DEFAULT_OUTPUT,
            componentManufacturer: au::kAudioUnitManufacturer_Apple,
            componentFlags: 0,
            componentFlagsMask: 0
        };

        unsafe {
            let comp = match au::AudioComponentFindNext(ptr::null_mut(), &desc as *const _) {
                x if x.is_null() => return Err(CoreAudioDriverError::AudioComponentNotFound),
                x => x
            };

            let mut instance: au::AudioComponentInstance = mem::uninitialized();
            check_os_error!(
                au::AudioComponentInstanceNew(comp, &mut instance as *mut _),
                CoreAudioDriverError::AudioComponentInstanceCreationFailed);

            check_os_error!(
                au::AudioUnitInitialize(instance),
                CoreAudioDriverError::AudioComponentInstanceInitializationFailed);

            let sample_rate = 44100;
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
                CoreAudioDriverError::AudioUnitSetPropertyFailed);

            let callback_ptr: *mut libc::c_void = mem::transmute(Box::new(callback));
            let render_callback = au::AURenderCallbackStruct {
                inputProc: Some(render_proc),
                inputProcRefCon: callback_ptr
            };
            check_os_error!(
                au::AudioUnitSetProperty(
                    instance,
                    au::kAudioUnitProperty_SetRenderCallback,
                    au::kAudioUnitScope_Input,
                    0,
                    &render_callback as *const _ as *const libc::c_void,
                    mem::size_of::<au::AURenderCallbackStruct>() as u32),
                CoreAudioDriverError::AudioUnitSetRenderCallbackFailed);

            check_os_error!(
                au::AudioOutputUnitStart(instance),
                CoreAudioDriverError::AudioOutputUnitStartFailed);

            Ok(CoreAudioDriver {
                instance: instance,
                callback: callback_ptr,
                is_enabled: true
            })
        }
    }
}

impl Drop for CoreAudioDriver {
    fn drop(&mut self) {
        unsafe {
            match au::AudioOutputUnitStop(self.instance) {
                err if err != 0 => panic!("Failed to stop audio output unit (error code {})", err),
                _ => {}
            }

            match au::AudioComponentInstanceDispose(self.instance) {
                err if err != 0 => panic!("Failed to dispose audio component instance (error code {})", err),
                _ => {}
            }

            let _: Box<Box<RenderCallback>> = mem::transmute(self.callback);
        }
    }
}

impl AudioDriver for CoreAudioDriver {
    fn set_render_callback(&mut self, callback: Box<RenderCallback>) {
        unsafe {
            let callback_ptr: *mut libc::c_void = mem::transmute(Box::new(callback));
            let render_callback = au::AURenderCallbackStruct {
                inputProc: Some(render_proc),
                inputProcRefCon: callback_ptr
            };
            if au::AudioUnitSetProperty(
                self.instance,
                au::kAudioUnitProperty_SetRenderCallback,
                au::kAudioUnitScope_Input,
                0,
                &render_callback as *const _ as *const libc::c_void,
                mem::size_of::<au::AURenderCallbackStruct>() as u32) != 0 {
                // TODO: Not sure if I like panicking here
                panic!("Failed to set render callback");
            }

            let _: Box<Box<RenderCallback>> = mem::transmute(self.callback);
            self.callback = callback_ptr;
        }
    }

    fn set_is_enabled(&mut self, is_enabled: bool) {
        if is_enabled == self.is_enabled {
            return;
        }

        unsafe {
            if is_enabled {
                match au::AudioOutputUnitStart(self.instance) {
                    // TODO: Not sure I like panicking here
                    err if err != 0 => panic!("Failed to stop audio output unit (error code {})", err),
                    _ => {}
                }
            } else {
                match au::AudioOutputUnitStop(self.instance) {
                    // TODO: Not sure I like panicking here
                    err if err != 0 => panic!("Failed to stop audio output unit (error code {})", err),
                    _ => {}
                }
            }
        }

        self.is_enabled = is_enabled;
    }

    fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    fn set_sample_rate(&mut self, sample_rate: i32) {
        let sample_rate_float = sample_rate as f64;
        unsafe {
            if au::AudioUnitSetProperty(
                self.instance,
                au::kAudioUnitProperty_SampleRate,
                au::kAudioUnitScope_Input,
                0,
                &sample_rate_float as *const _ as *const libc::c_void,
                mem::size_of::<f64>() as u32) != 0 {
                // TODO: Not sure I like panicking here
                panic!("Failed to set sample rate");
            }
        }
    }

    fn sample_rate(&self) -> i32 {
        unsafe {
            let mut sample_rate_float: f64 = 0.0;
            let mut size: u32 = mem::size_of::<f64>() as u32;
            if au::AudioUnitGetProperty(
                self.instance,
                au::kAudioUnitProperty_SampleRate,
                au::kAudioUnitScope_Input,
                0,
                &mut sample_rate_float as *mut _ as *mut libc::c_void,
                &mut size as *mut _) != 0 {
                // TODO: Not sure I like panicking here
                panic!("Failed to get sample rate");
            }
            sample_rate_float as i32
        }
    }
}

extern "C" fn render_proc(
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
}
