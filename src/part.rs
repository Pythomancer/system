use crate::geometry::*;
use crate::mesh::*;
use crate::sphere::BoundSphere;
pub enum Component {
    Root(Mesh),
    Assembly(Vec<Part>),
}
pub struct Part {
    pub origin: Point3,
    pub component: Component,
}

impl Part {
    pub fn bound_spheres(&self) -> Vec<BoundSphere> {
        let parts = self.part_tree_flatten();
        parts.iter().map(|x| x.bounds).collect()
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
            origin: Point3::origin(),
            component: Component::Root(mesh),
        }
    }
}
