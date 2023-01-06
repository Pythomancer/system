use std::iter::Enumerate;

use macroquad::{color, prelude::Color, rand::gen_range};

use crate::{geometry::*, matrix::Mat4, render::Renderer, sphere::BoundSphere};
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

    pub fn midpoint(&self, pts: &[Point3]) -> Point3 {
        Point3::sum_points(&[pts[self.a], pts[self.b], pts[self.c]]).scale(1.0 / 3.0)
    }

    pub fn normal(&self, pts: &[Point3]) -> Vec3 {
        let a = &pts[self.a];
        let b = &pts[self.b];
        let c = &pts[self.c];
        let ab = a.vec_to(b);
        let ac = a.vec_to(c);
        ab.cross(&ac)
    }

    pub fn norm_out(&self, pts: &[Point3]) -> Triangle {
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
    pub origin: Point3,
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
            for (i, idxo) in [1, 2, 4].iter().enumerate() {
                // index two
                for (j, idxt) in [6, 5, 3].iter().enumerate() {
                    if i != j {
                        tris.push(
                            Triangle {
                                a: idxz,
                                b: *idxo,
                                c: *idxt,
                                color: 4 * idxz + 2 * idxo + idxt,
                            }
                            .norm_out(&pts), // add a triangle for that face and make its normal outward manually,
                                             // as opposed to normal propagation
                        );
                        println!("{:?}", tris[tris.len() - 1]);
                        // println!("({}, {}, {})", idxz, idxo, idxt);
                    }
                }
            }
        }
        let bounds = BoundSphere::from_pts(&pts);
        Mesh {
            points: pts,
            triangles: tris,
            colors: vec![color::BLACK],
            bounds,
            origin: Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
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
    pub fn offset(&mut self, origin_offset: &Point3) {
        for pt in self.points.iter_mut() {
            pt.offset(origin_offset);
        }
    }
    pub fn offset_origin(&mut self, reverse: bool) {
        for p in &mut self.points {
            if reverse {
                p.offset(&self.origin.opposite());
            } else {
                p.offset(&self.origin);
            }
        }
    }
    pub fn transform(&mut self, mat: &Mat4) {
        for pt in self.points.iter_mut() {
            pt.transform(mat);
        }
    }
    pub fn mesh_color_randomise(&mut self) {
        self.colors = Vec::<color::Color>::new();
        for (i, tri) in self.triangles.iter_mut().enumerate() {
            self.colors.push(Mesh::random_color());
            tri.color = i;
        }
    }
    pub fn random_color() -> color::Color {
        Color {
            r: gen_range(0.0, 1.0),
            g: gen_range(0.0, 1.0),
            b: gen_range(0.0, 1.0),
            a: 1.0,
        }
    }
}
