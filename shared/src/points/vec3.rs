use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use forward_ref::{forward_ref_binop, forward_ref_op_assign};

use super::traits::{Modulo, ModuloAssign};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

forward_ref_binop!(impl Add, add for Vec3, Vec3);
forward_ref_op_assign!(impl AddAssign, add_assign for Vec3, Vec3);

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

forward_ref_binop!(impl Sub, sub for Vec3, Vec3);
forward_ref_op_assign!(impl SubAssign, sub_assign for Vec3, Vec3);

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

forward_ref_binop!(impl Mul, mul for Vec3, f64);
forward_ref_op_assign!(impl MulAssign, mul_assign for Vec3, f64);

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

forward_ref_binop!(impl Div, div for Vec3, f64);
forward_ref_op_assign!(impl DivAssign, div_assign for Vec3, f64);

impl Modulo<f64> for Vec3 {
    type Output = Vec3;

    fn modulo(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x % rhs,
            y: self.y % rhs,
            z: self.z % rhs,
        }
    }
}

impl ModuloAssign<f64> for Vec3 {
    fn modulo_assign(&mut self, rhs: f64) {
        self.x %= rhs;
        self.y %= rhs;
        self.z %= rhs;
    }
}

forward_ref_binop!(impl Modulo, modulo for Vec3, f64);
forward_ref_op_assign!(impl ModuloAssign, modulo_assign for Vec3, f64);

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

forward_ref_binop!(impl Mul, mul for Vec3, Vec3);
forward_ref_op_assign!(impl MulAssign, mul_assign for Vec3, Vec3);

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

forward_ref_binop!(impl Div, div for Vec3, Vec3);
forward_ref_op_assign!(impl DivAssign, div_assign for Vec3, Vec3);

impl Modulo<Vec3> for Vec3 {
    type Output = Vec3;

    fn modulo(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
            z: self.z % rhs.z,
        }
    }
}

impl ModuloAssign<Vec3> for Vec3 {
    fn modulo_assign(&mut self, rhs: Vec3) {
        self.x %= rhs.x;
        self.y %= rhs.y;
        self.z %= rhs.z;
    }
}

forward_ref_binop!(impl Modulo, modulo for Vec3, Vec3);
forward_ref_op_assign!(impl ModuloAssign, modulo_assign for Vec3, Vec3);
