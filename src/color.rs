pub use crate::vec3::Vec3;


pub type Color = Vec3;

impl Color { 
    pub fn write_color(&self){
        let ir = (255.999 * self.x()) as i32;
        let ig = (255.999 * self.y()) as i32;
        let ib = (255.999 * self.z()) as i32;
        println!("{} {} {}", ir, ig, ib);
    }

}