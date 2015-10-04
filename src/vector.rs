use super::{Numeric, Float, Zero};
use std;
//#[cfg(features = "unstable")]
//use std::num::Zero;
//use traits::Float;
use std::ops::{Index, IndexMut, Add, Mul};
//use std::ops::{Add, Sub, Mul, Div, Rem};


pub trait Vector<N>: Sized 
                   + Zero 
                   + Index<usize, Output = N> 
                   + IndexMut<usize, Output = N> 
    where N: Numeric 
           + Add<Output = N> 
           + Mul<Output = N> 
           + Copy
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
        where M: Float
    {
        (M::from(self.length_squared())).unwrap().sqrt()
    }
}

//#[cfg(features = "unstable")]
pub struct Vector3<N: Numeric> 
{
    values: [N; 3]
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

