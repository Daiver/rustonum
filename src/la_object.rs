use num::traits::Num;

pub trait LAObject<N: Num>: Sized + Clone {
    fn size(&self) -> usize;
}
