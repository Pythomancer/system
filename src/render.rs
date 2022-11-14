use crate::{
    geometry::{Point3, Vec3},
    mesh::{Mesh, Triangle},
    part::{self, Part},
    sphere::BoundSphere,
    utils::AngleRange,
};
use macroquad::{
    color,
    prelude::BLACK,
    prelude::RED,
    shapes::draw_line,
    window::{clear_background, screen_height, screen_width},
};

pub enum Renderer {
    Wireframe(color::Color),
    Simple(color::Color),
    Traced,
    None,
}
pub struct World {
    pub parts: Vec<part::Part>,
    pub camera: Camera,
    pub background: color::Color,
}

pub struct Camera {
    pub location: Point3,
    pub look: Vec3,
    pub hfov: f32,
    pub vfov: f32,
    pub renderer: Renderer,
}

impl Camera {
    pub fn to_ranges(&self) -> (AngleRange, AngleRange) {
        // (h, v)
        let slook = self.look.to_spherical();
        let h = AngleRange {
            min: slook.theta - self.hfov / 2.0,
            max: slook.theta + self.hfov / 2.0,
        };
        let v = AngleRange {
            min: slook.phi - self.vfov / 2.0,
            max: slook.phi + self.vfov / 2.0,
        };
        (h, v)
    }
    pub fn can_see(&self, bounds: &BoundSphere) -> bool {
        let view = self.to_ranges();
        let bound_view = bounds.angle_ranges_from(&self.location);
        !(view.0.overlap(bound_view.0).is_none() || view.1.overlap(bound_view.1).is_none())
    }
    pub fn pt_proj(&self, pt: &Point3) -> (f32, f32) {
        let n_look = self.look.norm();
        let flat = n_look.project_xy();
        let sidelook = Vec3 {
            x: -1.0 * flat.y,
            y: flat.x,
            z: 0.0,
        };
        let up = sidelook.cross(&n_look);
        let vec_to_pt = self.location.vec_to(pt).norm();
        let a_proj = vec_to_pt - n_look * n_look.dot(&vec_to_pt);
        (
            a_proj.dot(&sidelook) * screen_height() + 0.5 * screen_width(),
            (a_proj.dot(&up) + 0.5) * screen_height(),
        )
    }
    pub fn draw_tri(&self, tri: &Triangle, mesh: &Mesh) {
        match self.renderer {
            Renderer::Wireframe(color) => {
                let (a, b, c) = (
                    self.pt_proj(&mesh.points[tri.a]),
                    self.pt_proj(&mesh.points[tri.b]),
                    self.pt_proj(&mesh.points[tri.c]),
                );
                // println!("({},{}), ({},{}), ({},{})", a.0, a.1, b.0, b.1, c.0, c.1);
                draw_line(a.0, a.1, b.0, b.1, 1.0, color);
                draw_line(b.0, b.1, c.0, c.1, 1.0, color);
                draw_line(a.0, a.1, c.0, c.1, 1.0, color);
            }
            Renderer::None => {}
            _ => {}
        }
    }
    pub fn draw_mesh(&self, mesh: &Mesh) {
        for tri in &mesh.triangles {
            self.draw_tri(tri, mesh);
        }
    }
    pub fn draw_part(&self, part: &Part) {
        todo!()
    }
    pub fn new(color: color::Color) -> Camera {
        Camera {
            location: Point3::new(-2.0, 0.0, 0.0),
            look: Vec3::new(1.0, 0.0, 0.0),
            hfov: 100.0,
            vfov: 56.25,
            renderer: Renderer::Wireframe(color),
        }
    }
    pub fn tick(&self, time: u32) {}
}

impl World {
    pub fn render(&self) {
        clear_background(self.background);
        for p in &self.parts {
            self.camera.draw_part(p);
        }
    }

    pub fn tick(&mut self, time: u32) {
        for part in &mut self.parts {
            part.tick(time);
            self.camera.tick(time);
        }
    }
    pub fn new_empty() -> World {
        World {
            parts: Vec::new(),
            camera: Camera::new(RED),
            background: BLACK,
        }
    }
    pub fn add_part(&mut self, p: Part) {
        self.parts.push(p);
    }
}
