
mod vec3;
mod color;
mod ray;
use vec3::Vec3;
use ray::Ray;
use std::io::{self, Write};

//Image
const RATIO : f64 = 16.0 / 9.0;
const IMAGE_WIDTH : u16 = 400;
const IMAGE_HEIGHT : u16 = (IMAGE_WIDTH as f64 / RATIO) as u16;

fn main() {
    //Camera
    let view_height = 2.0;
    let view_width = RATIO * view_height;
    let focal = 1.0;
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(view_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, view_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0
            - vertical / 2.0  - Vec3::new(0.0, 0.0, focal);

    //Render 
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);    
    for j in (0..IMAGE_HEIGHT).rev(){
        eprintln!("\rScanlines remaining: {}",j);
        io::stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64)/((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64)/((IMAGE_HEIGHT- 1) as f64);
            let r = Ray::new(origin, lower_left_corner + horizontal * u 
                 + vertical * v - origin);
            let pixel_color = r.ray_color();
            pixel_color.write_color();
        }
    }
    eprintln!("\nDone.\n");
}
