use crate::{
    geometry::{Point3, Vec3},
    part,
    sphere::BoundSphere, utils::AngleRange,
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
    pub fn to_ranges(&self) -> (AngleRange, AngleRange) { // (h, v)
        let slook = self.look.to_spherical();
        let h = AngleRange {min: slook.theta - self.hfov/2.0, max: slook.theta + self.hfov/2.0};
        let v = AngleRange {min: slook.phi - self.vfov/2.0, max: slook.phi + self.vfov/2.0};
        (h, v)

    }
    pub fn can_see(&self, bounds: &BoundSphere) -> bool{
        let view = self.to_ranges();
        let bound_view = bounds.angle_ranges_from(&self.location);
        if view.0.overlap(bound_view.0).is_none() || 
            view.1.overlap(bound_view.1).is_none() { // if either angle does not match
                false
            } 
        else { 
            true
        }
    }
}

impl World {
    pub fn render(&self) {
        for m in self.meshes {
            if self.camera.can_see(m.bounds){
                
            }
        }
    }
}
