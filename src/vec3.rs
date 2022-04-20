use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;


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
    type Output = Self;
    fn add(self, rhs : Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs : f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}


impl Mul<Self> for Vec3 {
    type Output = Self;
    fn mul(self, rhs : Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Self;
    fn div(self, rhs : Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
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
        self.x * self.x + self.y * self.y + self.y * self.y 
    } 

    pub fn norm(&self) -> f64 {
        self.len().sqrt()
    }
}



