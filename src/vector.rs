use super::{Numeric, Zero};

//#[cfg(features = "unstable")]
//use std::num::Zero;
use std::ops::{Index, IndexMut, Add, Mul};
//use std::ops::{Add, Sub, Mul, Div, Rem};


//#[cfg(features = "unstable")]
pub trait Vector<N>: Sized + Zero + Index<usize, Output = N> + IndexMut<usize, Output = N> 
    where N: Numeric + Add<Output = N> + Mul<Output = N> + Copy
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

    //#[cfg(features = "unstable")]
    fn lengthSquared(&self) -> N 
    {
        self.dot(self)
    }

    //#[cfg(features = "unstable")]
    //fn length(&self) -> N 
    //{
        //self.lengthSquared().sqrt()
    //}

//    #[cfg(features = "unstable")]
    //fn is_perpendicular_to<M>(self, v_prime: Self) -> bool
    //where Self: Mul<Self, Output=M>
        //, M: PartialEq
    //{
        //(self * v_prime) == M::zero()
    //}
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

