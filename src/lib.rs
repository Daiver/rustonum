//#![crate_name = "rustonum"]
//#![crate_type = "lib"]
//#![cfg_attr(features = "unstable", feature(zero_one))]

pub use matrix::MatrixXf;
pub use numeric::{Zero, Numeric};

#[macro_use]
pub mod numeric;

#[macro_use]
pub mod matrix;

#[macro_use]
pub mod vector;

//not tested
pub mod geometry_primitives;

