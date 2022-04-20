use crate::vec3::Vec3;
use crate::color::Color;

#[derive(Debug)]
pub struct Ray  {
    origin : Vec3, // the origin points
    direction : Vec3,  // the direction vector
}

impl Ray {
    pub fn new(origin : Vec3, direction : Vec3) -> Self{
        Ray {
            origin : origin,
            direction : direction,
        }
    }
    
    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, t : f64) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn ray_color(&self) -> Color {
        if self.hit(Vec3::new(0.0, 0.0, -1.0), 0.5){
            return Color::new(1.0, 0.0, 0.0);
        }
        let unit_direction = self.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + 
            Color::new(0.5, 0.7, 1.0) * t
    }

    pub fn hit(&self, center : Vec3, radius : f64) -> bool{
        let oc  = *self.origin() - center; // vector from the center
        let a = self.direction().len();
        let b = 2.0 *(*self.direction() * oc);
        let c = oc.len() - radius * radius;
        let delta = b * b - 4.0 * a * c;
        delta > 0.0
    }
}