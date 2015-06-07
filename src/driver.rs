extern crate coreaudio_sys as bindings;
extern crate libc;

use std::ptr;
use std::mem;
use std::slice;
use self::bindings::audio_unit as au;

const COMPONENT_TYPE_OUTPUT: libc::c_uint = 0x61756f75;
const COMPONENT_SUB_TYPE_DEFAULT_OUTPUT: libc::c_uint = 0x64656620;

#[derive(Debug)]
pub enum DriverError {
    AudioComponentNotFound,
    AudioComponentInstanceCreationFailed,
    AudioComponentInstanceInitializationFailed,
    AudioUnitSetPropertyFailed,
    AudioUnitSetRenderCallbackFailed,
    AudioOutputUnitStartFailed
}

pub struct Driver {
    instance: au::AudioComponentInstance
}

macro_rules! check_os_error {
    ($expr:expr,$err:expr) => ({
        if $expr != 0 {
            return Err($err);
        }
    })
}

impl Driver {
    pub fn new(some_func: Box<FnMut(&mut[f32], usize)>) -> Result<Driver, DriverError> {
        let desc = au::AudioComponentDescription {
            componentType: COMPONENT_TYPE_OUTPUT,
            componentSubType: COMPONENT_SUB_TYPE_DEFAULT_OUTPUT,
            componentManufacturer: au::kAudioUnitManufacturer_Apple,
            componentFlags: 0,
            componentFlagsMask: 0
        };

        unsafe {
            let comp = match au::AudioComponentFindNext(ptr::null_mut(), &desc as *const _) {
                x if x.is_null() => return Err(DriverError::AudioComponentNotFound),
                x => x
            };

            let mut instance: au::AudioComponentInstance = mem::uninitialized();
            check_os_error!(
                au::AudioComponentInstanceNew(comp, &mut instance as *mut _),
                DriverError::AudioComponentInstanceCreationFailed);

            check_os_error!(
                au::AudioUnitInitialize(instance),
                DriverError::AudioComponentInstanceInitializationFailed);

            let mut stream_desc = au::AudioStreamBasicDescription {
                mSampleRate: 44100.0,
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
                DriverError::AudioUnitSetPropertyFailed);

            let callback = Box::new(RenderCallback {
                callback: some_func
            });

            let render_callback = au::AURenderCallbackStruct {
                inputProc: Some(input_proc), // TODO
                inputProcRefCon: mem::transmute(callback)
            };

            check_os_error!(
                au::AudioUnitSetProperty(
                    instance,
                    au::kAudioUnitProperty_SetRenderCallback,
                    au::kAudioUnitScope_Input,
                    0,
                    &render_callback as *const _ as *const libc::c_void,
                    mem::size_of::<au::AURenderCallbackStruct>() as u32),
                DriverError::AudioUnitSetRenderCallbackFailed);

            check_os_error!(
                au::AudioOutputUnitStart(instance),
                DriverError::AudioOutputUnitStartFailed);

            Ok(Driver {
                instance: instance
            })
        }
    }
}

impl Drop for Driver {
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
        }
    }
}

struct RenderCallback {
    callback: Box<FnMut(&mut[f32], usize)>
}

extern "C" fn input_proc(
    in_ref_con: *mut libc::c_void,
    _: *mut au::AudioUnitRenderActionFlags,
    _: *const au::AudioTimeStamp,
    _: au::UInt32,
    in_number_frames: au::UInt32,
    io_data: *mut au::AudioBufferList) -> au::OSStatus {
    let callback: *mut RenderCallback = in_ref_con as *mut _;
    unsafe {
        let slice_ptr = (*io_data).mBuffers[0].mData as *mut libc::c_float;
        let buffer = slice::from_raw_parts_mut(slice_ptr, (in_number_frames * 2) as usize);

        (*(*callback).callback)(buffer, in_number_frames as usize);
    }

    0
}
