use crate::geometry::{Point3, Vec4};

pub struct Mat4 {
    pub a: Vec4,
    pub b: Vec4,
    pub c: Vec4,
    pub d: Vec4,
}

impl Mat4 {
    pub fn from_arr(arr: [[f32; 4]; 4]) -> Mat4 {
        Mat4 {
            a: Vec4::from_arr(arr[0]),
            b: Vec4::from_arr(arr[1]),
            c: Vec4::from_arr(arr[2]),
            d: Vec4::from_arr(arr[3]),
        }
    }

    pub fn transpose(&self) -> Mat4 {
        Mat4 {
            a: Vec4 {
                x: self.a.x,
                y: self.b.x,
                z: self.c.x,
                w: self.d.x,
            },
            b: Vec4 {
                x: self.a.y,
                y: self.b.y,
                z: self.c.y,
                w: self.d.y,
            },
            c: Vec4 {
                x: self.a.z,
                y: self.b.z,
                z: self.c.z,
                w: self.d.z,
            },
            d: Vec4 {
                x: self.a.w,
                y: self.b.w,
                z: self.c.w,
                w: self.d.w,
            },
        }
    }

    pub fn transform(&self, input: Vec4) -> Vec4 {
        let transpose = self.transpose();
        Vec4 {
            x: input.dot(&transpose.a),
            y: input.dot(&transpose.b),
            z: input.dot(&transpose.c),
            w: input.dot(&transpose.d),
        }
    }

    pub fn mul(&self, other: &Mat4) -> Mat4 {
        let transpose = other.transpose();
        Mat4::from_arr([
            [
                self.a.dot(&transpose.a),
                self.a.dot(&transpose.b),
                self.a.dot(&transpose.c),
                self.a.dot(&transpose.d),
            ],
            [
                self.b.dot(&transpose.a),
                self.b.dot(&transpose.b),
                self.b.dot(&transpose.c),
                self.b.dot(&transpose.d),
            ],
            [
                self.c.dot(&transpose.a),
                self.c.dot(&transpose.b),
                self.c.dot(&transpose.c),
                self.c.dot(&transpose.d),
            ],
            [
                self.d.dot(&transpose.a),
                self.d.dot(&transpose.b),
                self.d.dot(&transpose.c),
                self.d.dot(&transpose.d),
            ],
        ])
    }

    pub fn translate(delta: Point3) -> Mat4 {
        Mat4::from_arr([
            [1.0, 0.0, 0.0, delta.x],
            [0.0, 1.0, 0.0, delta.y],
            [0.0, 0.0, 1.0, delta.z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn scale(factor: Point3) -> Mat4 {
        Mat4::from_arr([
            [factor.x, 0.0, 0.0, 0.0],
            [0.0, factor.y, 0.0, 0.0],
            [0.0, 0.0, factor.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotate_x(thx: f32) -> Mat4 {
        Mat4::from_arr([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, thx.cos(), -1.0 * thx.sin(), 0.0],
            [0.0, thx.sin(), thx.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotate_y(thy: f32) -> Mat4 {
        Mat4::from_arr([
            [thy.cos(), 0.0, thy.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-1.0 * thy.sin(), 0.0, thy.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotate_z(thz: f32) -> Mat4 {
        Mat4::from_arr([
            [thz.cos(), -1.0 * thz.sin(), 0.0, 0.0],
            [thz.sin(), thz.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotate(thx: f32, thy: f32, thz: f32) -> Mat4 {
        Mat4::rotate_x(thx).mul(&Mat4::rotate_y(thy).mul(&Mat4::rotate_z(thz)))
    }
}
