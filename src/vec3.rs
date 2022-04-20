use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Sub;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x : f64,
    y : f64,
    z : f64,
}

impl ToString for Vec3{
    fn to_string(&self) -> String {
        self.x.to_string() + " " +
         &self.y.to_string() + " " + &self.z.to_string()
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs : Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs : Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}


impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs : f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

// the inner product　
impl Mul<Vec3> for Vec3 {
    type Output = f64;
    fn mul(self, rhs : Vec3) -> Self::Output {
            self.x * rhs.x + 
            self.y * rhs.y + 
            self.z * rhs.z
    }
}


impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs : f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}




impl Vec3 {
    pub fn new(e0 : f64, e1 : f64, e2 :f64) -> Vec3 {
           Vec3{x : e0, y : e1, z : e2} 
    } //constructor

    pub fn x(&self) -> f64 {
        self.x
    } // x component

    pub fn y(&self) -> f64 {
        self.y
    }// y component

    pub fn z(&self) -> f64 {
        self.z
    }// z component
    
    pub fn len(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z 
    } 

    pub fn norm(&self) -> f64 {
        self.len().sqrt()
    }
    
    pub fn unit_vector(&self) -> Vec3 {
        *self / self.norm()
    }
}



