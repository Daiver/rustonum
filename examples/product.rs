#[macro_use] extern crate rustonum;

use rustonum::MatrixXf;

fn main()
{
    let m1 = mat![1.0, 2.0, 3.0; 4.0, 5.0, 6.0];
    let m2 = mat![-1.0, 3.0; 0.1, 0.2];
    let m3 = &m1.t() * &m2;// use references
    let m4 = m1.t() * m2;  // move m1 and m2
    println!("m3 = {:?}", m3);
    println!("diff between m3 and m4 {}", (m3 - m4).abs().sum());
}
