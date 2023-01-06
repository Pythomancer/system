use crate::geometry::*;
use crate::matrix::Mat4;
use crate::mesh::*;

pub struct Assembly {
    pub parts: Vec<Mesh>,
}
impl Assembly {
    pub fn transform(&mut self, mat: &Mat4) {
        for mesh in &mut self.parts {
            mesh.offset_origin(false); // make the point the actual point in space
            mesh.transform(mat); // transform it
            mesh.offset_origin(true); // put the point back to its own internal coordinate system
        }
    }
    pub fn from_mesh(m: Mesh) -> Assembly {
        Assembly {
            parts: Vec::from_iter([m]),
        }
    }
    pub fn add_mesh(&mut self, m: Mesh) {
        self.parts.push(m);
    }
}

impl Mesh {}
