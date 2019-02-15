extern crate cgmath;
use cgmath::{Vector3 as Vector3, Point3 as Point, InnerSpace, Zero};


use crate::scene::{Scene, Sphere, Plane, Element};

pub struct Ray{
    pub origin: Point<f64>,
    pub direction: Vector3<f64>,
}

impl Ray{
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray{
        // assumes width > height
        assert!(scene.width > scene.height);
        /**
         * trig to adjust fov
         *      (fov = left most ray to right most ray)
         */
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        /**
         * Calculute ratio and then multiply each x by aspect ratio
         */
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let sensor_x = (((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio * fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;
        /**
         * let pixel_center = x as f64 + 0.5;
         * let normalized_to_width = pixel_center / screen.width as f64;
         * let adjusted_screen_pos = (normalized_to_width * 2.0) - 1.0;
         */
        let sensor_x = ((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0; //Want it to be range of [-1.0, 1.0]
        let sensor_y = 1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0; //Convert from CS coordinates to real world coords

        Ray{
            origin: Point::<f64>{
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vector3::<f64>{
                x: sensor_x,
                y: sensor_y,
                z: -1.0,
            }
            .normalize(),
        }

        
    }
}

pub trait Intersectable{
    fn intersect(&self, ray: &Ray) -> Option<f64>;
}

impl Intersectable for Sphere{
    /**
     * use prime ray as X, current ray as R
     *      iff Y < Sphere.radius then
     *          current ray hits sphere
     * 
     *      iff Y == Sphere.radius then
     *          current ray tangent sphere
     * 
     *      iff Y < Sphere.radius then
     *          current ray not hit
     */
    fn intersect(&self, ray: &Ray) -> Option<f64>{
        //Create a line segment between the ray origin and the center of the sphere
        let l: Vector3<f64> = self.center - ray.origin;
        //Use l as a hypotenuse and find the length of the adjacent side
        let adj2 = l.dot(ray.direction);
        //Find the length-squared of the opposite side
        //This is equivalent to (but faster than) (l.length() * l.length()) - (adj2 * adj2)
        let d2 = l.dot(l) - (adj2 * adj2);
        //If that length-squared is less than radius squared, the ray intersects the sphere
        if d2 > (self.radius * self.radius){
            return None;
        }
        let radius2 = self.radius * self.radius;

        let thc = (radius2 - d2).sqrt();

        let t0 = adj2 - thc;
        let t1 = adj2 + thc;

        if t0 < 0.0 && t1 < 0.0{
            return None;
        }

        let distance = if t0 < t1 { t0 } else { t1 };
        Some(distance)
    }
}

pub trait Surface_Normal{
    fn surface_normal(&self, hit_point: &Point<f64>) -> Vector3<f64>;
}

impl Surface_Normal for Element{
    fn surface_normal(&self, hit_point: &Point<f64>) -> Vector3<f64> {
        match *self{
            Element::Sphere(ref s) => (*hit_point - s.center).normalize(),
            Element::Plane(ref p) => -p.normal,
        }
    
    }
    
    
}