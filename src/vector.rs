use num::traits::{Float, Num, NumCast};
use std::ops::{Index, IndexMut, Add, Sub, Mul, Div};
use super::Tensor;
//use std::fmt;

pub trait Vector<N>: Sized
                   + Tensor
                   + Index<usize, Output = N> 
                   + IndexMut<usize, Output = N> 
                   
    where N: Num
           + Add<Output = N> 
           + Mul<Output = N> 
           + Copy
           + NumCast,
    <Self as Index<usize>>::Output: Num
{
    //fn zero()
    //fn size(&self) -> usize;

    fn sum(&self) -> N
    {
        let mut res = N::zero();
        for i in (0 .. self.size()){
            res = res + self[i];
        }
        res
    }

    fn dot(&self, _rhs: & Self) -> N
    {
        assert!(self.size() == _rhs.size());
        let mut res = N::zero();
        for i in (0..self.size()){
            res = res + self[i] * _rhs[i];
        }
        res
    }

    fn length_squared(&self) -> N 
    {
        self.dot(self)
    }

}

//impl<N> fmt::Display for Vector<N>
    //where N: fmt::Display {
        ////FIXME: should be rewrited
    //fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    //{
        //try!(write!(f, "["));
        //for i in (0 .. self.size()) {
            //try!(write!(f, "{}", self[i]));
            //if i < self.size() - 1 {
                //try!(write!(f, ", "));
            //}
        //}
        //write!(f, "]");
    //}
//}

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
        for i in (0..self.size()) {
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

impl<N: Copy + Float + NumCast> Tensor for Vector3<N> {
    fn size(&self) -> usize {3}
}

impl<N> Vector<N> for Vector3<N> 
    where N: Float + Copy + NumCast {   }

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


//Vector2 Should be rewrited by common type or macroses
#[derive(Debug, Clone, Copy)]
pub struct Vector2<N: Float + Copy + NumCast> 
{
    pub values: [N; 2]
}

impl<N> Vector2<N>
    where N: Float + Copy + NumCast {
    pub fn x(&self) -> N {self[0]}
    pub fn y(&self) -> N {self[1]}

    pub fn zero() -> Self
    {
        Vector2{values: [N::zero(), N::zero()]}
    }

}

impl<N: Copy + Float + NumCast> Tensor for Vector2<N> {
    fn size(&self) -> usize {2}
}

impl<N> Vector<N> for Vector2<N> 
    where N: Float + Copy + NumCast {   }

impl<N> GeometryVector<N> for Vector2<N> 
    where N: Float + Copy + NumCast {}

impl<N> Index<usize> for Vector2<N> where N: Float + Copy + NumCast {
    type Output = N;
    fn index<'a>(&'a self, _index: usize) -> &'a N 
    {
        assert!(_index < 2);
        &self.values[_index]
    }
}

impl<N> IndexMut<usize> for Vector2<N> where N: Float + Copy + NumCast {
    fn index_mut<'a>(&'a mut self, _index: usize) -> &'a mut N {
        assert!(_index < 2);
        & mut self.values[_index]
    }
}



macro_rules! impl_static_vector_operators {
    ($($struct_name:ident, $trait_name:ident, $fun:ident)*) => {$(

    impl<N> $trait_name<$struct_name<N>> for $struct_name<N> 
        where N: Float + Copy + $trait_name<Output = N> {
        type Output = Self;

        fn $fun(self, _rhs: Self) -> Self {
            let mut res = self.clone();
            for i in (0 .. self.size()) {
                res[i] = res[i].$fun(_rhs[i]);
            }
            res
        }
    }

    impl<N> $trait_name<N> for $struct_name<N> 
        where N: Float + Copy + $trait_name<Output = N> {
        type Output = Self;

        fn $fun(self, _rhs: N) -> Self {
            let mut res = self.clone();
            for i in (0 .. self.size()) {
                res[i] = res[i].$fun(_rhs);
            }
            res
        }
    }
)*}}

macro_rules! impl_static_vector_operators_for_scalars {
    ($($struct_name:ident, $scalar_type_name:ident, $trait_name:ident, $fun:ident)*) => {$(

    impl $trait_name<$struct_name<$scalar_type_name>> for $scalar_type_name {
        type Output = $struct_name<$scalar_type_name>;
        fn $fun(self: $scalar_type_name,
                vec : $struct_name<$scalar_type_name>) -> $struct_name<$scalar_type_name>
        {
            let mut res = vec.clone();
            for i in (0 .. vec.size()) {
                res[i] = res[i].$fun(self);
            }
            res
        }
    }
       

)*}}

impl_static_vector_operators!{
    Vector3, Add, add
    Vector3, Mul, mul
    Vector3, Sub, sub
    Vector3, Div, div
}

impl_static_vector_operators!{
    Vector2, Add, add
    Vector2, Mul, mul
    Vector2, Sub, sub
    Vector2, Div, div
}

impl_static_vector_operators_for_scalars!{
    Vector3, f32, Add, add
    Vector3, f32, Sub, sub
    Vector3, f32, Mul, mul
    Vector3, f32, Div, div
}

impl_static_vector_operators_for_scalars!{
    Vector3, f64, Add, add
    Vector3, f64, Sub, sub
    Vector3, f64, Mul, mul
    Vector3, f64, Div, div
}

impl_static_vector_operators_for_scalars!{
    Vector2, f32, Add, add
    Vector2, f32, Sub, sub
    Vector2, f32, Mul, mul
    Vector2, f32, Div, div
}

impl_static_vector_operators_for_scalars!{
    Vector2, f64, Add, add
    Vector2, f64, Sub, sub
    Vector2, f64, Mul, mul
    Vector2, f64, Div, div
}

pub type Vector3f = Vector3<f32>;
pub type Vector3d = Vector3<f64>;

pub type Vector2f = Vector2<f32>;
pub type Vector2d = Vector2<f64>;

