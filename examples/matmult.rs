extern crate gel;
extern crate num;

use gel::Mat4;
use num::One;

fn main() {
    let mut x = Mat4::one();
    x = x * 3.0;
    let mut y = Mat4::one();
    y[3][0] = 3.0;
    y[3][1] = 2.0;
    y[3][2] = 4.0;

    let z = x * y;
    println!("{:?}", z);
}
