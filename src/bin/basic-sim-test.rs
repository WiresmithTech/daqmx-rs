/// Quick Test Program To Get A Value From A Channel

use daqmx;
fn main() {
    println!("{:?}", daqmx::get_value());
}