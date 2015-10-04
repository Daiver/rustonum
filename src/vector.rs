use num::traits::{Float, Num, FromPrimitive, NumCast};
use std::ops::{Index, IndexMut, Add, Mul};


pub trait Vector<N>: Sized
                   + Clone
                   + Index<usize, Output = N> 
                   + IndexMut<usize, Output = N> 
                   
    where N: Num
           + Add<Output = N> 
           + Mul<Output = N> 
           + Copy
           + NumCast
{
    //fn zero()
    fn count(&self) -> usize;

    fn dot(&self, _rhs: & Self) -> N
    {
        assert!(self.count() == _rhs.count());
        let mut res = N::zero();
        for i in (0..self.count()){
            res = res + self[i] * _rhs[i];
        }
        res
    }

    fn length_squared(&self) -> N 
    {
        self.dot(self)
    }

    fn length<M>(&self) -> M
        where M: Float + FromPrimitive
    {
        (M::from(self.length_squared())).unwrap().sqrt()
    }

}

pub trait GeometryVector<N> : Vector<N>
    where N: Num
           + Add<Output = N> 
           + Mul<Output = N> 
           + Copy
           + NumCast
{

}

#[derive(Debug, Clone, Copy)]
pub struct Vector3<N: Num + Copy + NumCast> 
{
    pub values: [N; 3]
}

impl<N> Vector<N> for Vector3<N> 
    where N:Num + Copy + NumCast {
    fn count(&self) -> usize {3}
}

impl<N> GeometryVector<N> for Vector3<N> 
    where N:Num + Copy + NumCast {}

impl<N> Index<usize> for Vector3<N> where N: Num + Copy + NumCast {
    type Output = N;
    fn index<'a>(&'a self, _index: usize) -> &'a N 
    {
        assert!(_index < 3);
        &self.values[_index]
    }
}

impl<N> IndexMut<usize> for Vector3<N> where N: Num + Copy + NumCast {
    fn index_mut<'a>(&'a mut self, _index: usize) -> &'a mut N {
        assert!(_index < 3);
        & mut self.values[_index]
    }
}

pub type Vector3f = Vector3<f32>;
pub type Vector3d = Vector3<f64>;
