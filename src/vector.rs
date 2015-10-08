use num::traits::{Float, Num, NumCast};
use std::ops::{Index, IndexMut, Add, Sub, Mul, Div};


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

    fn sum(&self) -> N
    {
        let mut res = N::zero();
        for i in (0 .. self.count()){
            res = res + self[i];
        }
        res
    }

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

}

pub trait GeometryVector<N> : Vector<N>
                            + Index<usize, Output = N> 
                            + IndexMut<usize, Output = N> 
    where N: Num
           + Add<Output = N> 
           + Mul<Output = N> 
           + Div<Output = N> 
           + Copy
           + NumCast 
           + Float
{ 
    fn length(&self) -> N
    {
        self.length_squared().sqrt()
    }

    fn normalize(& mut self)
    {
        let len = self.length();
        for i in (0..self.count()) {
            self[i] = self[i] / len;
        }
    }

    fn normalized(&self) -> Self
    {
        let mut res = self.clone();
        res.normalize();
        res
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector3<N: Float + Copy + NumCast> 
{
    pub values: [N; 3]
}

impl<N> Vector3<N>
    where N: Float + Copy + NumCast {
    pub fn x(&self) -> N {self[0]}
    pub fn y(&self) -> N {self[1]}
    pub fn z(&self) -> N {self[2]}

    pub fn zero() -> Self
    {
        Vector3{values: [N::zero(), N::zero(), N::zero()]}
    }

    pub fn cross(&self, other: Self) -> Self
    {
        Vector3{values:[
            self.y() * other.z() - self.z() * other.y(),
          -(self.x() * other.z() - self.z() * other.x()),
            self.x() * other.y() - self.y() * other.x()
        ]}
    }
}

impl<N> Vector<N> for Vector3<N> 
    where N: Float + Copy + NumCast {
    fn count(&self) -> usize {3}
}

impl<N> GeometryVector<N> for Vector3<N> 
    where N: Float + Copy + NumCast {}

impl<N> Index<usize> for Vector3<N> where N: Float + Copy + NumCast {
    type Output = N;
    fn index<'a>(&'a self, _index: usize) -> &'a N 
    {
        assert!(_index < 3);
        &self.values[_index]
    }
}

impl<N> IndexMut<usize> for Vector3<N> where N: Float + Copy + NumCast {
    fn index_mut<'a>(&'a mut self, _index: usize) -> &'a mut N {
        assert!(_index < 3);
        & mut self.values[_index]
    }
}

pub type Vector3f = Vector3<f32>;
pub type Vector3d = Vector3<f64>;


macro_rules! impl_static_vector_ops {
    ($($struct_name:ident, $name:ident, $fun:ident)*) => {$(

    impl<N> $name<$struct_name<N>> for $struct_name<N> 
        where N: Float + Copy + $name<Output = N> {
        type Output = Self;

        fn $fun(self, _rhs: Self) -> Self {
            let mut res = self.clone();
            for i in (0 .. self.count()) {
                res[i] = res[i].$fun(_rhs[i]);
            }
            res
        }//stringify
    }

    impl<N> $name<N> for $struct_name<N> 
        where N: Float + Copy + $name<Output = N> {
        type Output = Self;

        fn $fun(self, _rhs: N) -> Self {
            let mut res = self.clone();
            for i in (0 .. self.count()) {
                res[i] = res[i].$fun(_rhs);
            }
            res
        }//stringify
    }
)*}}

impl_static_vector_ops!{
    Vector3, Add, add
    Vector3, Mul, mul
    Vector3, Sub, sub
    Vector3, Div, div
}


impl Mul<Vector3f> for f32 {
    type Output = Vector3f;
    fn mul(self: f32, vec: Vector3f) -> Vector3f
    {
        Vector3f {values: [
            vec.x() * self,
            vec.y() * self,
            vec.z() * self
        ]}
    }
}

