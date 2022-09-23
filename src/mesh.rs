use macroquad::color;

use crate::{
    geometry::*,
    render::{Camera, Renderer},
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
        Mesh {
            points: pts,
            triangles: tris,
            colors: vec![color::BLACK],
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

pub struct BoundBox {
    pub min: Point3,
    pub max: Point3,
}

impl BoundBox {
    pub fn bb_from_mesh(mesh: &Mesh) -> BoundBox {
        let mut min_pt = mesh.points[0];
        let mut max_pt = mesh.points[0];
        min_pt.x = mesh
            .points
            .iter()
            .min_by_key(|pt| FloatOrd(pt.x))
            .unwrap()
            .x;
        min_pt.y = mesh
            .points
            .iter()
            .min_by_key(|pt| FloatOrd(pt.y))
            .unwrap()
            .y;
        max_pt.z = mesh
            .points
            .iter()
            .min_by_key(|pt| FloatOrd(pt.z))
            .unwrap()
            .z;
        min_pt.x = mesh
            .points
            .iter()
            .max_by_key(|pt| FloatOrd(pt.x))
            .unwrap()
            .x;
        max_pt.y = mesh
            .points
            .iter()
            .max_by_key(|pt| FloatOrd(pt.y))
            .unwrap()
            .y;
        max_pt.z = mesh
            .points
            .iter()
            .max_by_key(|pt| FloatOrd(pt.z))
            .unwrap()
            .z;
        BoundBox {
            min: min_pt,
            max: max_pt,
        }
    }

    pub fn is_visible(&self, camera: &Camera) -> bool {
        for x in [self.min.x, self.max.x] {
            for y in [self.min.y, self.max.y] {
                for z in [self.min.z, self.max.z] {
                    if (Vec3 { x, y, z } - camera.location.to_vec())
                        .to_spherical()
                        .is_in(&camera.look.to_spherical(), camera.hfov, camera.vfov)
                    {
                        return true;
                    }
                }
            }
        }
        false
    }
}

pub struct BoundSphere {
    center: Point3,
    radius: f32,
}

impl BoundSphere {
    pub fn from_mesh(mesh: Mesh) -> BoundSphere {
        match mesh.points.len() {
            0 => BoundSphere {
                center: Point3::origin(),
                radius: 0.0,
            },
            _ => {
                let c = Point3::sum_points(&mesh.points).scale(1.0 / mesh.points.len() as f32);
                let r = mesh
                    .points
                    .into_iter()
                    .max_by_key(|x| FloatOrd(x.vec_to(&c).mag()))
                    .expect("mesh is empty")
                    .vec_to(&c)
                    .mag();
                BoundSphere {
                    center: c,
                    radius: r,
                }
            }
        }
    }
}
