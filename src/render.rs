use crate::{
    geometry::{Point3, Vec3},
    part,
};
use macroquad::color;

pub enum Renderer {
    Wireframe(color::Color),
    Simple(color::Color),
    Traced,
    None,
}
pub struct World {
    pub meshes: Vec<part::Part>,
    pub renderer: Renderer,
    pub camera: Camera,
}

pub struct Camera {
    pub location: Point3,
    pub look: Vec3,
    pub vfov: f32,
    pub aspect_ratio: f32,
}

impl World {
    pub fn render() {}
}