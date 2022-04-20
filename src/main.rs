
mod vec3;
mod color;
mod ray;
use color::Color;
use vec3::Vec3;
use std::io::{self, Write};

//Image
const RATIO : f64 = 16.0 / 9.0;
const IMAGE_WIDTH : u16 = 400;
const IMAGE_HEIGHT : u16 = (400 as f64 / RATIO) as u16;

fn main() {
    //Camera
    let view_height = 2.0;
    let view_width = RATIO * view_height;
    let focal = 1.0;
    let orgin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(view_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, view_height, 0.0);

    for j in (0..IMAGE_HEIGHT).rev(){
        eprintln!("\rScanlines remaining: {}",j);
        io::stderr().flush().unwrap();
        
        for i in 0..IMAGE_WIDTH {
            let r = (i as f64)/((IMAGE_WIDTH - 1) as f64);
            let g = (j as f64)/((IMAGE_HEIGHT- 1) as f64);
            let b = 0.25;
            let pixel = Color::new(r, g, b);
            pixel.write_color();
        }
    }
}
