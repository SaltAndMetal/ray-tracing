#![allow(non_snake_case, dead_code, unused_imports)]
mod vec3;
mod camera;
mod ray;
mod hit;
mod hittable;
mod material;

use crate::vec3::*;
use crate::camera::Camera;
use crate::ray::Ray;
use crate::hit::Hit;
use crate::hittable::{Hittable, Sphere};
use crate::material::{Material, Lambertian, Metal};

extern crate bmp;
use bmp::{Image, Pixel, px};

const GREY_LAM: Lambertian = Lambertian::new(Colour::new(0.5, 0.5, 0.5));
const YELLOW_LAM: Lambertian = Lambertian::new(Colour::new(0.8, 0.8, 0.0));
const GREY_MET: Metal = Metal::new(Colour::new(0.5, 0.5, 0.5), 0.001);
const GREY_MET2: Metal = Metal::new(Colour::new(0.5, 0.5, 0.5), 0.1);

fn main() {
    let WORLD: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere{ centre: Position::new(0.0, 0.0, -1.75), radius: 0.5, material: &GREY_LAM }), 
        Box::new(Sphere{ centre: Position::new(0.0, -100.5, -1.0), radius: 100.0, material: &YELLOW_LAM }), 
        Box::new(Sphere{ centre: Position::new(-1.25, 0.0, -1.0), radius: 0.5, material: &GREY_MET }), 
        Box::new(Sphere{ centre: Position::new(1.25, 0.0, -1.0), radius: 0.5, material: &GREY_MET2 })
        ];
    
    let width = 800;
    let height = 400;
    let origin = Position::new(0.0, 0.0, 0.0);
    let viewportWidth = 8.0;
    let viewportHeight = 4.0;
    let focalLength = 1.0;
    let mut camera = Camera{ width, height, pixels:vec![vec![Colour::blank(); width as usize]; height as usize], viewportWidth, viewportHeight, lowerLeftCorner: Position::new(origin.x()-viewportWidth/2.0, origin.y()+viewportHeight/2.0, -focalLength), origin};

    for w in 0..width {
        for h in 0..height {
            camera.write_colour(w, h, 50, &WORLD);
        }
    }
    create_bmp(&camera);
}

fn create_bmp(camera: &Camera) {
    let mut img = Image::new(camera.width, camera.height);
    for (x, y) in img.coordinates() {
        let x = x as usize;
        let y = y as usize;
        img.set_pixel(x as u32, y as u32, px!((camera.pixels[y][x].r().powf(0.5_f64)*255.0) as u32, (camera.pixels[y][x].g().powf(0.5_f64)*255.0) as u32, (camera.pixels[y][x].b().powf(0.5_f64)*255.0) as u32));
    }
    let _ = img.save("image.bmp");
}
