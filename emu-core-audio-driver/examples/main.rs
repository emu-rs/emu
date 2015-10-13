extern crate emu_audio_types;
extern crate emu_core_audio_driver;

use emu_audio_types::audio_driver::{AudioDriver, RenderCallback};
use emu_core_audio_driver::core_audio_driver::CoreAudioDriver;

use std::f64::consts::PI;

use std::thread;

struct TestUserResource {
    name: String,
    phase: f64
}

impl TestUserResource {
    fn new(name: String) -> TestUserResource {
        println!("Test user resource created ({})", name);
        TestUserResource { name: name, phase: 0.0 }
    }
}

impl Drop for TestUserResource {
    fn drop(&mut self) {
        println!("Test user resource destroyed ({})", self.name);
    }
}

fn main() {
    let mut driver = {
        let mut test_user_resource = TestUserResource::new(String::from("a"));
        let callback: Box<RenderCallback> = Box::new(move |buffer, num_frames| {
            for i in 0..num_frames {
                let value = (test_user_resource.phase * PI).sin() as f32;
                let buffer_index = i * 2;
                buffer[buffer_index + 0] = value;
                buffer[buffer_index + 1] = value;
                test_user_resource.phase += 440.0 / 44100.0;
            }
        });

        // TODO: Ugly
        let mut ret = CoreAudioDriver::new();
        ret.set_render_callback(Some(callback));
        ret
    };

    println!("All systems are go.");

    println!("Starting render callback tests.");
    thread::sleep_ms(1000);

    println!("Swapping callback...");

    {
        let mut test_user_resource = TestUserResource::new(String::from("b"));
        let callback: Box<RenderCallback> = Box::new(move |buffer, num_frames| {
            for i in 0..num_frames {
                let value = (test_user_resource.phase * 2.0 * PI).sin() as f32;
                let buffer_index = i * 2;
                buffer[buffer_index + 0] = value;
                buffer[buffer_index + 1] = value;
                test_user_resource.phase += 440.0 / 44100.0;
            }
        });

        driver.set_render_callback(Some(callback));
    }

    println!("Callback swapped");
    thread::sleep_ms(1000);

    println!("Render callback tests completed.");

    println!("Starting is enabled tests.");

    println!("Driver is enabled: {}", driver.is_enabled());
    thread::sleep_ms(1000);

    driver.set_is_enabled(false);
    println!("Driver is enabled: {}", driver.is_enabled());
    thread::sleep_ms(1000);

    driver.set_is_enabled(true);
    println!("Driver is enabled: {}", driver.is_enabled());
    thread::sleep_ms(1000);

    println!("Is enabled tests completed.");

    println!("Starting sample rate tests.");

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
