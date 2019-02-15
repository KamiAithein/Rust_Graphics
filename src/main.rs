extern crate cgmath;
use cgmath::{Point3 as Point, InnerSpace, Vector3};

extern crate image;
use image::{DynamicImage, RgbImage, Rgba, Pixel, GenericImage};

use std::ops::Mul;

pub mod scene;
use crate::scene::{Scene, Sphere, Intersection, Light, Element};

mod rendering;
use crate::rendering::{Ray, Intersectable, Surface_Normal};

use std::path::Path;



pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0,0,0,0);
    for x in 0..scene.width{
        for y in 0..scene.height{
            let ray = Ray::create_prime(x, y, scene);
            for sphere in scene.elements.iter(){
                let intersect = sphere.intersect(&ray);
                if(intersect != None){
                    image.put_pixel(x, y, get_color(scene, &ray, &Intersection::new(sphere.intersect(&ray).unwrap(), sphere)));
                }
                
            }
        }
    }
    image.save(Path::new("./hi.png"));
    image
}

#[test]
fn test_can_render_scene() {
    let mut vec: Vec<Element> = Vec::new();
    vec.push(
        Element::Sphere(Sphere{
            center: Point {
                x: 0.0,
                y: 1.0,
                z: -10.0,
            },
            radius: 1.0,
            color: Rgba{
                data: [125, 125, 0, 125],
            },
            albedo: 1.0,
            })
    );
    vec.push(
        Element::Sphere(Sphere{
            center: Point {
                x: 10.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 2.0,
            color: Rgba{
                data: [0, 125, 0, 125],
            },
            albedo: 125.0,
            })
    );
    vec.push(
        Element::Sphere(Sphere{
            center: Point {
                x: 2.9,
                y: 1.0,
                z: -5.0,
            },
            radius: 2.0,
            color: Rgba{
                data: [0, 0, 255, 125],
            },
            albedo: 255.0,
            })
    );
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        elements: vec,
        light: Light{
            color: Rgba::from_channels(255, 255, 255, 125),
            direction: Vector3::new(2.0, 2.0, 2.0),
            intensity: 1.0,
        }
    };

    let img: DynamicImage = render(&scene);
    let rgb: RgbImage = img.to_rgb();
    let (rgb_width, rgb_height) = rgb.dimensions();

    assert_eq!(scene.width, rgb_width);
    assert_eq!(scene.height, rgb_height);
}

fn main(){
    let mut vec: Vec<Element> = Vec::new();
    vec.push(
        Element::Sphere(Sphere{
            center: Point {
                x: -4.0,
                y: -2.0,
                z: -20.0,
            },
            radius: 7.5,
            color: Rgba{
                data: [0, 125, 0, 125],
            },
            albedo: 100.0,
            })
    );
    vec.push(
        Element::Sphere(Sphere{
            center: Point {
                x: 0.0,
                y: 1.0,
                z: -10.0,
            },
            radius: 1.0,
            color: Rgba{
                data: [125, 125, 0, 125],
            },
            albedo: 1.0,
            })
    );
    
    vec.push(
        Element::Sphere(Sphere{
            center: Point {
                x: 2.9,
                y: 1.0,
                z: -5.0,
            },
            radius: 2.0,
            color: Rgba{
                data: [0, 0, 255, 125],
            },
            albedo: 0.75,
            })
    );
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        elements: vec,
        light: Light{
            color: Rgba::from_channels(255, 255, 255, 255),
            direction: Vector3::new(2.0, 0.0, 2.0),
            intensity: 2000.0,
        }
    };
    
    render(&scene);
}

fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection) -> Rgba<u8> {
    let hit_point = ray.origin + (ray.direction * intersection.distance);
    let surface_normal = intersection.element.surface_normal(&hit_point);
    let direction_to_light = -scene.light.direction.normalize();
    let light_power = (surface_normal.dot(direction_to_light) as f32).max(0.0) *
                      scene.light.intensity;
    let light_reflected = intersection.element.albedo() / std::f32::consts::PI as f64;

    let i = intersection.element.color().clone().data;

    let (ir, ig, ib, ia) = (i[0], i[1], i[2], i[3]);

    let s = scene.light.color.clone().data;

    let (sr, sg, sb, sa) = (s[0], s[1], s[2], s[3]);

    let mut r = ir as f64 + sr as f64 + light_power as f64 + light_reflected;
    let mut g = ig as f64 + sg as f64 + light_power as f64 + light_reflected;
    let mut b = ib as f64 + sb as f64 + light_power as f64 + light_reflected;
    r /= 4.0;
    g /= 4.0;
    b /= 4.0;


    
    Rgba::from_channels(r as u8, g as u8, b as u8,125)
}