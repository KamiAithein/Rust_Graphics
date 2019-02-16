use image::{ Pixel, Rgba};
use std::ops::{Mul,};

#[derive(Clone, Copy)]
pub struct Color{
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}
impl Color{
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Color{

        let within_range = |val| -> bool{val <= 1.0 && val >= 0.0};

        if !within_range(r) || !within_range(g) || !within_range(b) || !within_range(a){
            panic!("Color out of range [0.0, 1.0]: r:{}, g:{}, b:{}, a:{}", r, g, b, a);
        }

        Color{
            r,
            g,
            b,
            a,
        }
    }
}

impl Color{
    pub fn to_rgba(&self) -> Rgba<u8>{
        Rgba::from_channels(
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            255,
        )
    }
    pub fn clamp(&self) -> Color {
        Color {
            r: self.r.min(1.0).max(0.0),
            b: self.b.min(1.0).max(0.0),
            g: self.g.min(1.0).max(0.0),
            a: 0.5,
        }
    }
    
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            b: self.b * other.b,
            g: self.g * other.g,
            a: 0.5,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
            a: 0.5,
        }
    }
}

#[test]
#[should_panic]
fn test_invalid_over(){
    let _color = Color::new(2.0, 2.0, 2.0, 2.0); 
}

#[test]
#[should_panic]
fn test_invalid_under(){
    let _color = Color::new(-0.1, -0.1, -0.1, -0.1);
}

#[test]
fn test_valid(){
    let (r, g, b, a) = (0.0, 0.1, 0.9, 1.0);
    let color = Color::new(r, g, b, a);

    assert_eq!(color.r, r);
    assert_eq!(color.g, g);
    assert_eq!(color.b, b);
    assert_eq!(color.a, a);
}

