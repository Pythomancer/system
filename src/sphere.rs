use crate::geometry::*;
use crate::utils::AngleRange;
use float_ord::FloatOrd;
pub struct BoundSphere {
    center: Point3,
    radius: f32,
}

impl BoundSphere {
    pub fn from_pts(pts: &Vec<Point3>) -> BoundSphere {
        match pts.len() {
            0 => BoundSphere {
                center: Point3::origin(),
                radius: 0.0,
            },
            _ => {
                let c = Point3::sum_points(pts).scale(1.0 / pts.len() as f32);
                let r = pts
                    .iter()
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
    pub fn view_cone_angle_radius(&self, pt: &Point3) -> f32 {
        self.radius.atan2(self.center.vec_to(pt).mag())
    }
    pub fn angle_ranges_from(&self, pt: &Point3) -> (AngleRange, AngleRange) {
        // (h, v)
        let phi = pt.vec_to(&self.center).to_spherical().phi; // NOTE must be vec from pt (camera) to center, otherwise theta will be off
        let theta = pt.vec_to(&self.center).to_spherical().theta;
        let vcar = self.view_cone_angle_radius(pt);
        let h = AngleRange {
            min: theta - vcar,
            max: theta + vcar,
        };
        let v = AngleRange {
            min: phi - vcar,
            max: phi + vcar,
        };
        (h, v)
    }
    pub fn offset(&mut self, center: Point3) {
        self.center += center
    }
}

pub struct SphereCircle {
    pub c: SpherePoint,
    pub theta_r: f32,
    pub phi_r: f32,
}

impl SphereCircle {}

pub struct SpherePoint {
    pub theta: f32,
    pub phi: f32,
    pub r: f32,
}

impl SpherePoint {
    pub fn is_in(&self, other: &SpherePoint, hfov: f32, vfov: f32) -> bool {
        (self.theta - other.theta).abs() < hfov && (self.phi - other.phi).abs() < vfov
    }
}
