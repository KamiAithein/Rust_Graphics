extern crate image as im;
use im::{DynamicImage, RgbImage, Rgba, Pixel, GenericImage, ImageBuffer};

extern crate piston_window;

use piston_window::*;

use piston_window::types::Color as p_color;

use std::path::Path;

mod scene;
use crate::scene::*;

mod vector3;
use crate::vector3::*;

mod color;
use crate::color::*;

mod point;
use crate::point::*;

mod rendering;
use crate::rendering::*;

//pub fn render(scene: &Scene, c: &Context, g: &mut G2d, window: PistonWindow) -> DynamicImage {
    
    //image.save(Path::new("./hi.png"));
    //image
//}

fn test_scene(frames: f64) -> Scene{
    let mut vec: Vec<Drawable> = Vec::new();
    
    vec.push(
        Drawable::Sphere(Sphere{
            center: Point {
                x: -1.0 + frames/10.0,
                y: -0.5,
                z: -10.0,
            },
            radius: 4.0,
            color: Color {
                r: 0.0,
                g: 1.0,
                b: 0.0,
                a: 0.5,
            },
            albedo: 1.0,
            })
    );
    
    vec.push(
        Drawable::Sphere(Sphere{
            center: Point {
                x: 0.0,
                y: -0.5 + frames/100.0,
                z: -5.0,
            },
            radius: 1.0,
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 1.0,
                a: 1.0,
            },
            albedo: 1.0,
            })
    );
    vec.push(
        Drawable::Sphere(Sphere{
            center: Point {
                x: 2.0 - frames/200.0,
                y: -0.5,
                z: -2.0 - frames/10.0,
            },
            radius: 0.5,
            color: Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 0.5,
            },
            albedo: 1.0,
            })
    );
    
    Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        drawables: vec,
        light: Light{
            color: Color{
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 0.5,
            },
            direction: Vector3::new(-2.0 + frames/10.0, -2.0 + frames, -2.0 + frames/10.0),
            intensity: 5.0,
        }
    }
}

fn main() {
    let mut frame = 0.0;
    let mut scene = test_scene(frame);

    let (width, height) = (1000, 900);
    let mut window: PistonWindow = WindowSettings::new(
        "fractal",
        [
            width,
            height,
        ],
    ).exit_on_esc(true).build().unwrap();

    //let mut image = DynamicImage::new_rgb8(scene.width, scene.height);

    let mut imgbuf: im::ImageBuffer<im::Rgba<u8>, _> = im::ImageBuffer::new(scene.width as u32, scene.height as u32);

    let mut texture: G2dTexture = Texture::from_image(
        &mut window.factory,
        &imgbuf,
        &TextureSettings::new()
    ).unwrap();
    let black = Rgba::from_channels(0,0,0,0);
    
    while let Some(event) = window.next(){
        if let Some(_) = event.render_args(){
            imgbuf = im::ImageBuffer::new(scene.width as u32, scene.height as u32);
            frame += 1.0;
            scene = test_scene(frame);
            for x in 0..scene.width{
                for y in 0..scene.height{
                    let ray = Ray::create_prime(x, y, &scene);
                    let intersect = scene.trace(&ray);
                        match intersect {
                            // The division was valid
                            Some(intersection) => {
                                let color =  rendering::get_color(&scene, &ray, &intersection);
                                let rgba = color.to_rgba();
                                //image.put_pixel(x, y, rgba);
                                
                                imgbuf.put_pixel(x, y, rgba);
                            },
                            // The division was invalid
                            None    => {}
                        }
                }
            }
            texture.update(&mut window.encoder, &imgbuf).unwrap();
            window.draw_2d(&event, |c, g|{
            clear([0.0,0.0,0.0,0.0], g);
            image(&texture, c.transform, g);
        });
        }
        
        event.update(|args|{

        });
    }
}
    
