extern crate coreaudio_sys as bindings;
extern crate libc;

use std::ptr;
use std::mem;
use self::bindings::audio_unit as au;

const COMPONENT_TYPE_OUTPUT: libc::c_uint = 0x61756f75;
const COMPONENT_SUB_TYPE_DEFAULT_OUTPUT: libc::c_uint = 0x64656620;

#[derive(Debug)]
pub enum DriverError {
    AudioComponentNotFound,
    AudioComponentInstanceCreationFailed,
    AudioComponentInstanceInitializationFailed,
    AudioUnitSetPropertyFailed,
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
    pub fn new() -> Result<Driver, DriverError> {
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

            // TODO: Set callback
            // https://github.com/yupferris/FerrisLibs/blob/master/Fel/src/Win32DirectSoundAudioDriver.cpp
            // https://github.com/RustAudio/coreaudio-rs/blob/master/src/audio_unit/mod.rs
            // http://stackoverflow.com/questions/26577070/how-to-use-fn-traits-closures-in-signatures-in-rust

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
            // TODO: Handle errors (probably by panicking)
            au::AudioOutputUnitStop(self.instance);
            au::AudioComponentInstanceDispose(self.instance);
        }
    }
}
