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
    pub fn spheres (&self) -> Vec<BoundSphere>{
        let out = Vec::<BoundSphere>::new();
        match self.component {
            Component::Root(mesh) => {return vec![mesh.bounds]}
            Component::Assembly(parts) => {
                for part in &parts {
                    out.extend(part.spheres())    
                }
            }
        }
        out
    }
}