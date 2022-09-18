use crate::geometry;
use crate::geometry::Point3;
use crate::mesh::*;
use std::fmt;
use std::ops::Mul;
impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}
impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "idx: {}, {}, {}", self.a, self.b, self.c)
    }
}

impl fmt::Display for Point3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "idx: {}, {}, {}", self.x, self.y, self.z)
    }
}

impl fmt::Display for Mesh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "points: {:?} \ntriangles: {:?}",
            self.points, self.triangles
        )
    }
}
