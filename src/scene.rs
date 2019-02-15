extern crate cgmath;
use cgmath::{Point3 as Point, Vector3, InnerSpace};

extern crate image;
use image::{Rgba, Pixel};

use crate::rendering::{Intersectable, Ray,};

pub struct Sphere{
    pub center: Point<f64>,
    pub radius: f64,
    pub color: Rgba<u8>,
    pub albedo: f64,
}

pub struct Scene{
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub elements: Vec<Element>,
    pub light: Light,
}

impl Scene{
    pub fn trace(&self, ray: &Ray) -> Option<Intersection>{
        self.elements
            .iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d,s)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}

pub struct Intersection<'a>{
    pub distance: f64,
    pub element: &'a Element,
}

impl<'a> Intersection<'a>{
    pub fn new<'b>(distance: f64, element: &'b Element) -> Intersection<'b>{
        Intersection{
            distance,
            element,
        }
    }
}

//defined by a point and then the normal to that point
pub struct Plane{
    pub origin: Point<f64>,
    pub normal: Vector3<f64>,
    pub color: Rgba<u8>,
    pub albedo: f64,

}

pub enum Element{
    Sphere(Sphere),
    Plane(Plane),
}

impl Element{
    pub fn color(&self) ->Rgba<u8>{
        match *self{
            Element::Sphere(ref s) => s.color,
            Element::Plane(ref p) => p.color,
        }
    }
    pub fn albedo(&self) -> f64{
        match *self{
            Element::Sphere(ref s) => s.albedo,
            Element::Plane(ref p) => 0.0,
        }
    }
}

impl Intersectable for Element{
    fn intersect(&self, ray: &Ray) -> Option<f64>{
        match *self{
            Element::Sphere(ref s) => s.intersect(ray),
            Element::Plane(ref p) => p.intersect(ray),
        }
    }
}
impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = self.normal;
        let denom = normal.dot(ray.direction);
        if denom > 1e-6 {
            let v = self.origin - ray.origin;
            let distance = v.dot(normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }
}

pub struct Light{
    pub direction: Vector3<f64>,
    pub color: Rgba<u8>,
    pub intensity: f32,
}
