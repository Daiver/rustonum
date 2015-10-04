
//extern crate num;

pub use matrix::MatrixXf;
pub use numeric::{Zero, Numeric, Float};

#[macro_use]
pub mod numeric;

#[macro_use]
pub mod matrix;

#[macro_use]
pub mod vector;

//not tested
pub mod geometry_primitives;

