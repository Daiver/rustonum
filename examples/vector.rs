#[macro_use] extern crate rustonum;

use rustonum::vector::*;

fn main()
{
    let v1 = vec3f![1.0, 2.0, 3.0];
    let v2 = vec3![5.0, 6.0, 7.0];
    let v3 = v1.cross(v2);

    println!("v3 = {:?}", v3);
    println!("length of v3 = {}", v3.length());
    println!("direction of v3 = {:?}", v3.normalized());
    println!("restored v3 = {:?}", v3.normalized() * v3.length());

    let v4 = vec3f![2.0, 3.0, 1.0];
    let v5 = vec3![5.0, 7.0, 3.0];
    let v6 = vec3![1.0, 1.0, 1.0];
    let v7 = 2.0 * (v4 + v6 * 0.5) - v5;
    println!("v7 = {}", v7.sum());
    let v8 = 0.1 * vec2![-1.0, 0.0] * vec2![1.0, 2.0];
    println!("v8 = {:?}", v8);
}
