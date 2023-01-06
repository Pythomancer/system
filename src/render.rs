use crate::{
    geometry::{Point3, Vec3},
    matrix::Mat4,
    mesh::{Mesh, Triangle},
    part::Assembly,
    sphere::BoundSphere,
    utils::AngleRange,
};
use macroquad::{
    color,
    math::Vec2,
    prelude::BLACK,
    prelude::{Color, RED},
    shapes::{draw_line, draw_triangle},
    window::{clear_background, screen_height, screen_width},
};

pub enum Renderer {
    Wireframe(color::Color),
    Simple(color::Color),
    Traced,
    None,
}
pub struct World {
    pub parts: Vec<Assembly>,
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
    pub fn draw_vec(&self, a: &Point3, b: &Point3) {
        let ap = self.pt_proj(a);
        let bp = self.pt_proj(b);
        draw_line(ap.0, ap.1, bp.0, bp.1, 1.0, RED);
    }
    pub fn draw_tri(&self, tri: &Triangle, mesh: &Mesh, tcolor: Color) {
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
            Renderer::Simple(_color) => {
                let n = tri.normal(&mesh.points);
                if n.dot(&self.location.vec_to(&tri.midpoint(&mesh.points))) < 0.0 {
                    let (a, b, c) = (
                        self.pt_proj(&mesh.points[tri.a]),
                        self.pt_proj(&mesh.points[tri.b]),
                        self.pt_proj(&mesh.points[tri.c]),
                    );
                    draw_triangle(
                        Vec2 { x: a.0, y: a.1 },
                        Vec2 { x: b.0, y: b.1 },
                        Vec2 { x: c.0, y: c.1 },
                        tcolor,
                    );
                    // DRAW NORMALS AT EDGES
                    // self.draw_vec(
                    //     &mesh.points[tri.a],
                    //     &(mesh.points[tri.a] + &tri.normal(&mesh.points).norm()),
                    // );
                }
            }
            Renderer::None => {}
            _ => {}
        }
    }
    pub fn draw_mesh(&self, mesh: &Mesh) {
        for tri in &mesh.triangles {
            self.draw_tri(tri, mesh, mesh.colors[tri.color]);
        }
    }
    pub fn draw_part(&self, assembly: &Assembly) {
        for mesh in &assembly.parts {
            self.draw_mesh(mesh);
        }
    }
    pub fn new(color: color::Color) -> Camera {
        Camera {
            location: Point3::new(-4.0, 0.0, 0.0),
            look: Vec3::new(1.0, 0.0, 0.0),
            hfov: 100.0,
            vfov: 56.25,
            renderer: Renderer::Wireframe(color),
        }
    }
}

impl World {
    pub fn render(&self) {
        clear_background(self.background);
        for p in &self.parts {
            self.camera.draw_part(p);
        }
    }

    pub fn new_empty() -> World {
        World {
            parts: Vec::new(),
            camera: Camera::new(RED),
            background: BLACK,
        }
    }
    pub fn add_part(&mut self, p: Assembly) {
        self.parts.push(p);
    }
    pub fn transform_mesh_index(&mut self, i: usize, mat: Mat4) {
        self.parts[i].transform(&mat);
    }
}
