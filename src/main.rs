mod driver;

use std::io;
use driver::Driver;

fn main() {
    {
        let driver = match Driver::new() {
            Ok(x) => x,
            Err(e) => panic!("{:?}", e)
        };

        println!("All systems are go.");

        let mut derp = String::new();
        io::stdin().read_line(&mut derp).ok();
    }

    println!("Dropped successfully.");
}
