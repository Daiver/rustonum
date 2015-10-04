
//Taken from https://github.com/hawkw/lin/blob/master/src/lib.rs
//I learn a lot of intresting stuff from this

//use std::ops::{Add, Sub, Mul, Div, Rem};

//#[cfg(features = "unstable")]
//use std::num::Zero;

//#[cfg(features = "unstable")]
//pub trait Numeric: PartialEq + PartialOrd
                             //+ Add<Self>
                             //+ Sub<Self>
                             //+ Mul<Self>
                             //+ Div<Self>
                             //+ Rem<Self>
                             //+ Zero
                             //+ Sized {}

//pub trait Zero
//{
    //fn zero() -> Self;
//}

////#[cfg(not(features = "unstable"))]
//pub trait Numeric: PartialEq + PartialOrd
                             //+ Add<Self>
                             //+ Sub<Self>
                             //+ Mul<Self>
                             //+ Div<Self>
                             //+ Rem<Self>
                             //+ Zero
                             //+ Sized {}

//pub trait NumCast<T>: Sized {
    //fn from(x: T) -> Option<Self>;
//}

//pub trait Float: Sized + NumCast<Self> {
    //fn sqrt(&self) -> Self;
//}

////FIXME Should be rewrited
//impl NumCast<f32> for f32 {
    //fn from(x: f32) -> Option<f32>
    //{
        //Some(x)
    //}
//}

//impl NumCast<u32> for f32 {
    //fn from(x: u32) -> Option<f32>
    //{
        //Some(x as f32)
    //}
//}

//impl Float for f32 {
    //fn sqrt(&self) -> f32
    //{
        //self.sqrt()
    //}
//}

//macro_rules! make_zero {
    //($($t:ty, $e: expr);*) => { 
        //$(impl Zero for $t {
            //fn zero() -> $t{ $e }
        //})* 
//};}

//macro_rules! make_numeric {
    //($($t:ty)*) => { $(impl Numeric for $t {})* };
//}

//make_zero!(u8,0; f32,0.0);

//make_numeric!(u8 f32);
////make_numeric!(u8 u16 u32 u64 usize i8 i16 i32 i64 isize f32 f64);
