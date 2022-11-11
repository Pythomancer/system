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
    pub tick_fn: fn(&Self, time: u32),
}

impl Part {
    pub fn default_tick_fn(&self, time: u32) {}
    pub fn bound_spheres(&self) -> Vec<BoundSphere> {
        let parts = self.part_tree_flatten();
        parts.iter().map(|x| x.bounds).collect()
    }
    pub fn transform(&self, mat: &Mat4) {
        for p in self.part_tree_flatten() {
            p.transform(mat);
        }
    }
    pub fn part_tree_flatten(&self) -> Vec<Mesh> {
        let mut out = Vec::<Mesh>::new();
        match &self.component {
            Component::Root(mesh) => return vec![mesh.offset(&self.origin)],
            Component::Assembly(parts) => {
                for part in parts {
                    out.extend(part.part_tree_flatten())
                }
            }
        }
        out
    }

    pub fn from_mesh(mesh: Mesh) -> Part {
        Part {
            tick_fn: Self::default_tick_fn,
            origin: Point3::origin(),
            component: Component::Root(mesh),
        }
    }
    pub fn tick(&self, time: u32) {
        (self.tick_fn)(self, time);
    }
}
