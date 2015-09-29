#[macro_use] extern crate rustonum;

use rustonum::MatrixXf;

fn main() {
    let m1 = mat![1.0, 2.0, 3.0; 4.0, 5.0, 6.0];
    let m2 = MatrixXf::consts(2, 3, -5.0);
    println!("{:?}", m1 + m2);

}
