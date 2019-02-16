use std::ops::{Neg, Mul,};


#[derive(Clone, Copy)]
pub struct Vector3{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3{
    pub fn new(x: f64, y: f64, z:f64) -> Vector3{
        Vector3{
            x,
            y,
            z,
        }
    }
}

impl Vector3{
    //sum of squares
    pub fn norm(&self) -> f64{
        let square = |val| -> f64{val * val};
        square(self.x) + square(self.y) + square(self.z)
    }

    pub fn length(&self) -> f64{
        self.norm().sqrt()
    }

    //Converts vector to unit length of 1 while maintaining angle
    pub fn normalize(&self) -> Vector3{
        let len = self.length().recip();

        Vector3{
            x: self.x * len,
            y: self.y * len,
            z: self.z * len,
        }
    }

    pub fn dot(&self, other: &Vector3) -> f64{
        self.x * other.x + self.y * other.y + self.z * other.z 
    }
}

impl Neg for Vector3{
    type Output = Vector3;

    fn neg(self) -> Vector3{
        Vector3{
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for Vector3{
    type Output = Vector3;

    fn mul(self, other: f64) -> Vector3{
        Vector3{
            x: self.x*other,
            y: self.y*other,
            z: self.z*other,
        }
    }
}

#[test]
fn test_normalize(){
    let original = Vector3{x: 10.0, y: 2.0, z: -1.0};
    let normal = original.normalize();

    let square = |val| {val * val};

    let unit_length = (square(normal.x) + square(normal.y) + square(normal.z)).sqrt();

    assert!(unit_length >= 1.0 - 0.1 && unit_length <= 1.0 + 0.1);
}

#[test]
fn test_dot() {
    let (lx, ly, lz) = (10.0, -10.0, 0.0);
    let left = Vector3{x: lx, y: ly, z: lz};

    let (rx, ry, rz) = (1.0, -1.0, 100.0);
    let right = Vector3{x: rx, y: ry, z: rz};

    assert_eq!(left.dot(&right), lx*rx + ly*ry + lz*rz);
}