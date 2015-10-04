
#[macro_use] extern crate num;

pub use matrix::MatrixXf;

pub use vector::{Vector, GeometryVector, Vector3f};

//#[macro_use]
//pub mod numeric;

#[macro_use]
pub mod matrix;

#[macro_use]
pub mod vector;

//not tested
pub mod geometry_primitives;

