use crate::scene::*;
use crate::point::*;
use crate::vector3::*;
use crate::color::*;

//shoots out from an origin
pub struct Ray{
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray{
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray{
        //assumption for ease: most monitors aren't taller than they are wide
        assert!(scene.width > scene.height);

        /**
         * Deals with the fact that the monitor and the scene have different ratios
         *      where the fov is the angle between 2 outer most opposing rays (i.e. left most and right most)
         */
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        
        /**
         * Calculute ratio of scene and then multiply each x by aspect ratio
         */

        //Ratio of scene
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let sensor_x = (((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio* fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;
        /**
         * let pixel_center = x as f64 + 0.5;
         * let normalized_to_width = pixel_center / screen.width as f64;
         * let adjusted_screen_pos = (normalized_to_width * 2.0) - 1.0;
         */
        //let sensor_x = ((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0; //Want it to be range of [-1.0, 1.0]
        //let sensor_y = 1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0; //Convert from CS coordinates to real world coords

        Ray{
            //Where the ray originates from (camera defaults to be at (0,0,0))
            origin: Point{
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vector3{
                x: sensor_x,
                y: sensor_y,
                z: -1.0
            }
            .normalize()
        }
    }
}

//distance away from a drawable
pub struct Intersection<'a>{
    pub distance: u32,
    pub drawable: &'a Drawable,
}

impl<'a> Intersection<'a>{
    pub fn new<'b>(distance: u32, drawable: &'b Drawable) -> Intersection<'b>{
        Intersection{
            distance,
            drawable,
        }
    }
}

pub trait Intersectable{
    fn intersect(&self, ray: &Ray) -> Option<f64>;
}

impl Intersectable for Sphere{
    /**
     * use prime ray as X, current ray as R in right triangle
     *      iff Y <= Sphere.radius then
     *          current ray hits sphere
     * 
     *      iff Y < Sphere.radius then
     *          current ray not hit
     */
    fn intersect(&self, ray: &Ray) -> Option<f64>{
        //Create a line segment between the ray origin and the center of the sphere
        let l: Vector3 = self.center - ray.origin;
        //Use l as a hypotenuse and find the length of the adjacent side
        let adj2 = l.dot(&ray.direction);
        //Find the length-squared of the opposite side
        //This is equivalent to (but faster than) (l.length() * l.length()) - (adj2 * adj2)
        let d2 = l.dot(&l) - (adj2 * adj2);
        //If that length-squared is less than radius squared, the ray intersects the sphere
        let rad2 = (self.radius*self.radius) as f64;
        
        if d2 > (rad2){
            return None;
        }

        let thc = (rad2 - d2).sqrt();

        let t0 = adj2 - thc;
        let t1 = adj2 + thc;

        if t0 < 0.0 && t1 < 0.0{
            return None;
        }

        let distance = if t0 < t1 { t0 } else { t1 };
        Some(distance)
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            let v = self.origin - ray.origin;
            let distance = v.dot(&normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }
}

impl Intersectable for Drawable{
    fn intersect(&self, ray: &Ray) -> Option<f64>{
        match *self{
            Drawable::Sphere(ref s) => s.intersect(ray),
            Drawable::Plane(ref p) => p.intersect(ray),
        }
    }
}

pub trait Surface_Normal{
    fn surface_normal(&self, hit_point: &Point) -> Vector3;
}

impl Surface_Normal for Drawable{
    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        match *self{
            Drawable::Sphere(ref s) => (*hit_point - s.center).normalize(),
            Drawable::Plane(ref p) => -p.normal,
        }
    }
}

pub fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection) -> Color {
    let hit_point = ray.origin + (ray.direction * intersection.distance as f64);

    let surface_normal = intersection.drawable.surface_normal(&hit_point);

    let direction_to_light = -scene.light.direction.normalize();

    let light_power = (surface_normal.dot(&direction_to_light)).max(0.0) * scene.light.intensity;
    let light_reflected = intersection.drawable.albedo() / std::f32::consts::PI as f64;

    let mut color = intersection.drawable.color().clone();

    (color*scene.light.color.clone()*light_power*light_reflected).clamp()
}

impl Scene{
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.drawables
            .iter()
            .filter_map(|e| e.intersect(ray).map(|d| Intersection::new(d as u32, e)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}
