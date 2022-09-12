use crate::geometry;
use std::ops::Mul;


impl Mul for geometry::Vec4 {
    type Output = f32;

    fn mul(self, rhs: Self) -> f32 {
        self.x * rhs.x + 
        self.y * rhs.y + 
        self.z * rhs.z + 
        self.w * rhs.w  
    }
}

impl Sub for 