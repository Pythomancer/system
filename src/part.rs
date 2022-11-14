use crate::geometry::*;
use crate::matrix::Mat4;
use crate::mesh::*;
use crate::sphere::BoundSphere;
pub enum Component {
    Root(Mesh),
    Assembly(Vec<Part>),
}
pub struct Part {
    pub origin: Point3,
    pub component: Component,
    pub tick_fn: fn(&mut Part, u32),
}

impl Part {
    pub fn default_tick_fn(p: &mut Part, time: u32) {}
    pub fn transform(&self, mat: &Mat4, offset: Point3) {
        match self.component {
            Component::Root(m) => m.offset(&offset).transform(mat),
            Component::Assembly(parts) => {
                for part in &parts {
                    part.transform(mat, offset + part.origin)
                }
            }
        }
    }

    pub fn from_mesh(mesh: Mesh) -> Part {
        Part {
            tick_fn: Self::default_tick_fn,
            origin: Point3::origin(),
            component: Component::Root(mesh),
        }
    }
    pub fn tick(&mut self, time: u32) {
        (self.tick_fn)(self, time);
    }
}
