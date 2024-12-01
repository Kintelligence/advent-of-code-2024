use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
pub struct Vector3d {
    pub  x: f64,
    pub  y: f64,
    pub  z: f64,
}

impl Vector3d {
    pub fn cross(&self, rhs: &Vector3d) -> Vector3d {
        Vector3d {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
    
    pub  fn dot(&self, rhs: &Vector3d) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul<f64> for &Vector3d {
    type Output = Vector3d;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3d {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<f64> for Vector3d {
    type Output = Vector3d;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3d {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

// Implement Add for &Vector3d
impl Add for &Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

// Implement Add for Vector3d
impl Add for Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

// Implement Add for &Vector3d and Vector3d
impl Add<Vector3d> for &Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: Vector3d) -> Self::Output {
        Vector3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

// Implement Add for Vector3d and &Vector3d
impl Add<&Vector3d> for Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: &Vector3d) -> Self::Output {
        Vector3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3d {
    type Output = Vector3d;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub for &Vector3d {
    type Output = Vector3d;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<Vector3d> for &Vector3d{
    type Output = Vector3d;

    fn sub(self, rhs: Vector3d) -> Self::Output {
        Vector3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<&Vector3d> for Vector3d{
    type Output = Vector3d;

    fn sub(self, rhs: &Vector3d) -> Self::Output {
        Vector3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Div<f64> for Vector3d {
    type Output = Vector3d;

    fn div(self, rhs: f64) -> Self::Output {
        Vector3d {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<f64> for &Vector3d {
    type Output = Vector3d;

    fn div(self, rhs: f64) -> Self::Output {
        Vector3d {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
