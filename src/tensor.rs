
pub trait Tensor: Sized + Clone {
    fn size(&self) -> usize;
}
