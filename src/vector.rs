//#[macro_use] extern crate num;

//use super::{Numeric, Zero};
//use std;
//#[cfg(features = "unstable")]
//use std::num::Zero;
use num::traits::{Float, Num, FromPrimitive, NumCast};
use std::ops::{Index, IndexMut, Add, Mul};
//use std::ops::{Add, Sub, Mul, Div, Rem};


pub trait Vector<N>: Sized 
                   + Index<usize, Output = N> 
                   + IndexMut<usize, Output = N> 
                   
    where N: Num
           + Add<Output = N> 
           + Mul<Output = N> 
           + Copy
           + NumCast
{
    fn count(&self) -> usize;

    //#[cfg(features = "unstable")]
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

#[derive(Debug, Clone, Copy)]
pub struct Vector3<N: Num + Copy + NumCast> 
{
    values: [N; 3]
}

impl<N> Vector<N> for Vector3<N> 
    where N:Num + Copy + NumCast {
    fn count(&self) -> usize 
    {
        3
    }
}

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

//#[cfg(features = "unstable")]
//impl<N> Zero for Vector3<N> where N: Numeric {
    //fn zero(&self) -> Vector3<N> 
    //{
        //Vector3{values: [N::zero(), N::zero(), N::zero()]}
    //}
//}

//impl<N> Add<Vector<N>> for Vector<N> 
//where N: Numeric + Add<Output = N> {
    //type Output = Vector<N>;

    //#[cfg(features = "unstable")]
    //fn add(self, _rhs: Vector<N>) -> Vector<N>
    //{
        //let res = Vector::zero();
        //res
    //}
//}

