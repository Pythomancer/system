use macroquad::color;

use crate::{
    geometry::*,
    render::{Camera, Renderer}, sphere::BoundSphere,
};
use float_ord::FloatOrd;
#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub color: usize,
}

impl Triangle {
    pub fn flip_normal(&self) -> Triangle {
        Triangle {
            a: self.a,
            b: self.c,
            c: self.b,
            color: self.color,
        }
    }

    pub fn normal(&self, pts: &Vec<Point3>) -> Vec3 {
        let a = &pts[self.a];
        let b = &pts[self.b];
        let c = &pts[self.c];
        let ab = a.vec_to(b);
        let ac = a.vec_to(c);
        ab.cross(&ac)
    }

    pub fn norm_out(&self, pts: &Vec<Point3>) -> Triangle {
        let norm = self.normal(pts);
        let a = &pts[self.a];

        if norm.dot(&a.vec_to(&Point3::origin())) > 0.0 {
            self.flip_normal()
        } else {
            *self
        }
    }
}
pub struct Mesh {
    pub points: Vec<Point3>,
    pub triangles: Vec<Triangle>,
    pub colors: Vec<color::Color>,
    pub bounds: BoundSphere,
}

impl Mesh {
    pub fn cube(sl: f32) -> Mesh {
        let mut pts = Vec::<Point3>::new();
        let mut tris = Vec::<Triangle>::new();
        let v = vec![0.5 * sl, -0.5 * sl];
        for i in v.clone() {
            for j in v.clone() {
                for k in v.clone() {
                    // add one point for each combination of 1 and -1 (times 0.5*sl) in x y and z
                    pts.push(Point3 { x: i, y: j, z: k });
                }
            }
        }
        // triangles originate from points 0 and 7
        for idxz in [0, 7] {
            // index one, where index zero is zero
            for idxo in 1..7 {
                // index two
                for idxt in (idxo + 1)..7 {
                    tris.push(
                        Triangle {
                            a: idxz,
                            b: idxo,
                            c: idxt,
                            color: 0,
                        }
                        .norm_out(&pts), // add a triangle for that face and make its normal outward manually,
                                         // as opposed to normal propagation
                    );
                }
            }
        }
        let bounds = BoundSphere::from_pts(&pts);
        Mesh {
            points: pts,
            triangles: tris,
            colors: vec![color::BLACK],
            bounds
        }
    }

    pub fn draw(&self, renderer: &Renderer) {
        match renderer {
            Renderer::Wireframe(color) => {
                println!("{:?}", color);
            }
            Renderer::Simple(color) => {
                println!("{:?}", color);
            }
            _ => {}
        }
    }
}
