mod driver;

use std::io;
use driver::Driver;

use std::f64::consts::PI;

fn main() {
    let mut phase: f64 = 0.0;

    let _driver = match Driver::new(Box::new(move |buffer, num_frames| {
        for frame in 0..num_frames {
            let sample = (phase * 2.0 * PI).sin() as f32;
            for channel in buffer.iter_mut() {
                channel[frame] = sample;
            }
            phase += 440.0 / 44100.0;
        }
        Ok(())
    })) {
        Ok(x) => x,
        Err(e) => panic!("{:?}", e)
    };

    println!("All systems are go.");

    let mut derp = String::new();
    io::stdin().read_line(&mut derp).ok();
}
