use crate::geometry::*;
pub struct Part {
    pub origin: Vec4,
    pub subparts: Vec<Part>,
}