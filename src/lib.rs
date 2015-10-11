#[macro_use] extern crate num;

pub use matrix::MatrixXf;

pub use vector::{Vector, GeometryVector, Vector3, Vector3f, Vector2f};

#[macro_use]
pub mod matrix;

#[macro_use]
pub mod vector;

#[macro_use] pub mod helper_macroses;
