use crate::point::{Point,};

use crate::color::{Color,};

use crate::vector3::{Vector3,};




//What is being drawn
pub struct Scene{
    pub drawables: Vec<Drawable>,
    pub light: Light,
    pub fov: f64,
    pub width: u32,
    pub height: u32,
    
} 



pub struct Sphere{
    pub center: Point,
    pub radius: f64,
    pub color: Color,
    pub albedo: f64,
}

//defined by a point and then a vector normal to that point
pub struct Plane{
    pub origin: Point,
    pub normal: Vector3,
    pub color: Color,
    pub albedo: f64,
}

pub struct Light{
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f64,
}

//all types of drawables
pub enum Drawable{
    Sphere(Sphere),
    Plane(Plane),
}

impl Drawable{
    pub fn color(&self) ->Color{
        match *self{
            Drawable::Sphere(ref s) => s.color,
            Drawable::Plane(ref p) => p.color,
        }
    }
    pub fn albedo(&self) -> f64{
        match *self{
            Drawable::Sphere(ref s) => s.albedo,
            Drawable::Plane(ref p) => 0.5,
        }
    }
}
