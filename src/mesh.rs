use crate::geometry::*;

pub struct Triangle {
    pub a: Point3,
    pub b: Vec4,
    pub c: Vec4,
}
pub struct Mesh {
    pub tris: Vec<Triangle>,
}
