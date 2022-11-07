use crate::{matrix::Mat4, sphere::*};
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn to_vec4(&self) -> Vec4 {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: 1.0,
        }
    }

    pub fn from_arr(varr: [f32; 3]) -> Vec3 {
        Vec3 {
            x: varr[0],
            y: varr[1],
            z: varr[2],
        }
    }

    pub fn to_pt3(&self) -> Point3 {
        Point3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x - other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn to_spherical(&self) -> SpherePoint {
        let rad = (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt();
        SpherePoint {
            theta: self.y.atan2(self.x),
            phi: (self.z / rad).acos(),
            r: rad,
        }
    }

    pub fn mag(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    pub fn scale(&self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn norm(&self) -> Self {
        *self / self.mag()
    }

    pub fn angle_between(&self, other: &Self) -> f32 {
        (self.dot(other) / self.mag() / other.mag()).acos()
    }

    pub fn project_xy(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: 0.0,
        }
    }
}

pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn to_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn from_arr(varr: [f32; 4]) -> Vec4 {
        Vec4 {
            x: varr[0],
            y: varr[1],
            z: varr[2],
            w: varr[3],
        }
    }

    pub fn to_pt3(&self) -> Point3 {
        Point3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn dot(&self, other: &Vec4) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: &Vec4) -> Vec4 {
        self.to_vec3().cross(&other.to_vec3()).to_vec4()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Point3 {
        Point3 { x, y, z }
    }
    pub fn matching_coords(&self, other: &Point3) -> usize {
        let mut i: usize = 0;
        if self.x == other.x {
            i += 1;
        }
        if self.y == other.y {
            i += 1;
        }
        if self.z == other.z {
            i += 1;
        }
        i
    }

    pub fn vec_to(&self, other: &Point3) -> Vec3 {
        Vec3 {
            x: other.x - self.x,
            y: other.y - self.y,
            z: other.z - self.z,
        }
    }

    pub fn to_vec(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn origin() -> Point3 {
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn scale(&self, scalar: f32) -> Point3 {
        Point3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn sum_points(pts: &Vec<Point3>) -> Point3 {
        let mut a = Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        for p in pts {
            a += *p;
        }
        a
    }
}
