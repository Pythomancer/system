use crate::geometry::*;
use crate::mesh::*;
pub enum Component {
    Root(Mesh),
    Assembly(Vec<Part>),
}
pub struct Part {
    pub origin: Point3,
    pub component: Component,
}
