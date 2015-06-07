mod driver;

use std::io;
use driver::Driver;

use std::f64::consts::PI;

fn main() {
    let mut phase: f64 = 0.0;
    let _driver = match Driver::new(Box::new(move |buffer, num_frames| {
        for i in 0..num_frames {
            let value = (phase * 2.0 * PI).sin() as f32;
            let buffer_index = i * 2;
            buffer[buffer_index + 0] = value;
            buffer[buffer_index + 1] = value;
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
