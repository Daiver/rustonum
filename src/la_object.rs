use std::ops::Index;

pub trait LAObject: Sized + Clone {
    fn size(&self) -> usize;
}
