mod driver;

use std::io;
use driver::Driver;

fn main() {
    let _driver = match Driver::new(Box::new(move |buffer, num_frames| {
        for frame in 0..num_frames {
            let sample = 0.0;
            for channel in buffer.iter_mut() {
                channel[frame] = sample;
            }
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
