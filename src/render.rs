use crate::{
    geometry::{Point3, Vec3},
    part,
    sphere::BoundSphere,
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
    pub hfov: f32,
    pub vfov: f32,
}

impl Camera {
    pub fn can_see(&self, bounds: &BoundSphere) {}
}

impl World {
    pub fn render() {}
}
