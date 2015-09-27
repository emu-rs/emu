mod audio_driver;
mod coreaudio_audio_driver;

use audio_driver::{AudioDriver, RenderCallback};
use coreaudio_audio_driver::CoreaudioAudioDriver;

use std::f64::consts::PI;

use std::thread;

struct TestUserResource {
    value: i32
}

impl TestUserResource {
    fn new(value: i32) -> TestUserResource {
        println!("Test user resource created ({})", value);
        TestUserResource { value: value }
    }
}

impl Drop for TestUserResource {
    fn drop(&mut self) {
        println!("Test user resource destroyed ({})", self.value);
    }
}

fn main() {
    let test_user_resource = TestUserResource::new(42);
    let mut phase: f64 = 0.0;
    let callback: Box<RenderCallback> = Box::new(move |buffer, num_frames| {
        let _ = test_user_resource;
        for i in 0..num_frames {
            let value = (phase * 2.0 * PI).sin() as f32;
            let buffer_index = i * 2;
            buffer[buffer_index + 0] = value;
            buffer[buffer_index + 1] = value;
            phase += 440.0 / 44100.0;
        }
    });

    let mut driver = match CoreaudioAudioDriver::new(callback) {
        Ok(x) => x,
        Err(e) => panic!("{:?}", e)
    };

    println!("All systems are go.");

    println!("Driver sample rate: {}", driver.sample_rate());
    thread::sleep_ms(1000);

    driver.set_sample_rate(32000);
    println!("Driver sample rate: {}", driver.sample_rate());
    thread::sleep_ms(1000);

    driver.set_sample_rate(22050);
    println!("Driver sample rate: {}", driver.sample_rate());
    thread::sleep_ms(1000);

    driver.set_sample_rate(11025);
    println!("Driver sample rate: {}", driver.sample_rate());
    thread::sleep_ms(1000);

    driver.set_sample_rate(96000);
    println!("Driver sample rate: {}", driver.sample_rate());
    thread::sleep_ms(1000);

    driver.set_sample_rate(44100);
    println!("Driver sample rate: {}", driver.sample_rate());
    thread::sleep_ms(1000);

    println!("Sample rate tests completed.");

    //let mut derp = String::new();
    //io::stdin().read_line(&mut derp).ok();
}
