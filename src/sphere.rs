use crate::geometry::*;
use crate::mesh::*;
use float_ord::FloatOrd;
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
    pub fn line_intersects(&self, line: Line3) -> bool {
        self.radius > line.dist_pt(&self.center)
    }
    pub fn angle_width_from(&self, pt: Point3) -> f32 {
        self.radius.atan2(self.center.vec_to(&pt).mag())
    }
}

pub struct SphereCircle {
    pub theta_c: f32,
    pub phi_c: f32,
    pub theta_r: f32,
    pub phi_r: f32,
}
pub struct SpherePoint3 {
    pub theta: f32,
    pub phi: f32,
    pub r: f32,
}

impl SpherePoint3 {
    pub fn is_in(&self, other: &SpherePoint3, hfov: f32, vfov: f32) -> bool {
        (self.theta - other.theta).abs() < hfov && (self.phi - other.phi).abs() < vfov
    }
}
