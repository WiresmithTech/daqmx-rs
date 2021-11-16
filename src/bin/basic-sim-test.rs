/// Quick Test Program To Get A Value From A Channel

use ni_daqmx;
fn main() {
    println!("{:?}", ni_daqmx::get_value());
}