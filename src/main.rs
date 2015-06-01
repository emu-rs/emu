mod driver;

use driver::Driver;

fn main() {
    {
        let driver = match Driver::new() {
            Ok(x) => x,
            Err(e) => panic!("{:?}", e)
        };
        println!("All systems are go.");
    }
    println!("Dropped successfully.");
}
