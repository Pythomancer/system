use crate::geometry::*;
#[derive(Clone, Copy)]
pub struct Triangle {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

impl Triangle {
    pub fn flip_normal(&self) -> Triangle {
        Triangle {
            a: self.a,
            b: self.c,
            c: self.b,
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

        match norm.dot(&a.vec_to(&Point3::origin())) > 0.0 {
            True => self.flip_normal(),
            False => *self,
        }
    }
}
pub struct Mesh {
    pub points: Vec<Point3>,
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            points: Vec::new(),
            triangles: Vec::new(),
        }
    }
    pub fn cube(sl: f32) {
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
                        }
                        .norm_out(&pts), // add a triangle for that face and make its normal outward manually,
                                         // as opposed to normal propagation
                    );
                }
            }
        }
    }
}
