use std::ops::{Add, Sub, Mul};

#[derive(Clone, Copy, Debug)]
pub struct Vector2f {
    pub x : f32,
    pub y : f32,
}


#[derive(Debug, Clone, Copy)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3f {
    pub fn cross(&self, other: Vector3f) -> Vector3f
    {
        Vector3f{
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x}
    }

    pub fn dot(&self, v: Vector3f) -> f32
    {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn length_squared(&self) -> f32
    {
        self.dot(*self)
    }

    pub fn length(&self) -> f32
    {
        self.length_squared().sqrt()
    }
    
    pub fn normalized(&self) -> Vector3f
    {
        let len = self.length();
        Vector3f {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len
        }
    }
}

impl Add for Vector3f {
    type Output = Vector3f;

    fn add(self, _rhs: Vector3f) -> Vector3f {
        Vector3f {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl Sub for Vector3f {
    type Output = Vector3f;

    fn sub(self, _rhs: Vector3f) -> Vector3f {
        Vector3f {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl Mul<f32> for Vector3f {
    type Output = Vector3f;
    fn mul(self, factor: f32) -> Vector3f
    {
        Vector3f {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor
        }
    }
}

impl Mul<Vector3f> for f32 {
    type Output = Vector3f;
    fn mul(self: f32, vec: Vector3f) -> Vector3f
    {
        Vector3f {
            x: vec.x * self,
            y: vec.y * self,
            z: vec.z * self
        }
    }
}


