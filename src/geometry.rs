pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
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

    // pub fn plane(&self) -> Plane {
    //     if self.y == 0.0 && self.z == 0.0 {
    //         Plane::X(self.x)
    //     } else if self.x == 0.0 && self.z == 0.0 {
    //         Plane::Y(self.y)
    //     } else if self.y == 0.0 && self.x == 0.0 {
    //         Plane::Z(self.z)
    //     } else {
    //         Plane::None
    //     }
    // }
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

pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub enum Plane {
    X(f32),
    Y(f32),
    Z(f32),
    None,
}

impl Point3 {
    pub fn distance_mag(&self, other: &Point3) -> f32 {
        ((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0) + (self.z - other.z).powf(2.0))
            .sqrt()
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

    pub fn to_vec(&self) -> Vec4 {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: 1.0,
        }
    }

    pub fn coplane(a: &Point3, b: &Point3, c: &Point3) -> Plane {
        if a.x == b.x && b.x == c.x {
            Plane::X(a.x)
        } else if a.y == b.y && b.y == c.y {
            Plane::Y(a.y)
        } else if a.z == b.z && b.z == c.z {
            Plane::Z(a.z)
        } else {
            Plane::None
        }
    }

    pub fn origin() -> Point3 {
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
