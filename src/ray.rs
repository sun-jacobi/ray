use crate::vec3::Vec3;

pub struct Ray {
    orgin : Vec3, // the origin points
    direction : Vec3,  // the direction vector
}

impl Ray {
    pub fn new(orgin : Vec3, direction : Vec3) -> Self{
        Ray {
            orgin : orgin,
            direction : direction,
        }
    }
    
    pub fn orgin(self) -> Vec3 {
        self.orgin
    }

    pub fn direction(self) -> Vec3 {
        self.direction
    }

    pub fn at(self, t : f64) -> Vec3 {
        self.orgin + self.direction * t
    }
    
    
}