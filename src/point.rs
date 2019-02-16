use std::ops::{Add, Sub};
use crate::vector3::*;

#[derive(Clone, Copy)]
pub struct Point{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point{
    pub fn new(x: f64, y: f64, z: f64) -> Point{
        Point{
            x,
            y,
            z,
        }
    }
}

impl Sub<Point> for Point{
    type Output = Vector3;

    fn sub(self, other: Point) -> Vector3{
        Vector3{
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add<Vector3> for Point{
    type Output = Point;

    fn add(self, other: Vector3) -> Point{
        Point{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<Point> for Vector3{
    type Output = Point;

    fn add(self, other: Point) -> Point{
        other + self
    }
    
}