use std::ops;

// Vec3 struct
#[derive(Copy, Clone)]
pub struct Vec3(pub f64,pub f64,pub f64);

pub use Vec3 as Color;

pub use Vec3 as Point3;

// pub type Color = Vec3;

// pub type Point3 = Vec3;

impl Vec3 {
    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        Vec3(
            self.0 / self.length(),
            self.1 / self.length(),
            self.2 / self.length(),
        )
    }

    pub fn print(&self) {
        print!("{} {} {}", self.0, self.1, self.2)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 = self.0 * rhs;
        self.1 = self.1 * rhs;
        self.2 = self.2 * rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        let m = 1.0 / rhs;
        self * m
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        let m = 1.0 / rhs;
        *self *= m;
    }
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    return Vec3(
        u.1 * v.2 - u.2 * v.1,
        u.2 * v.0 - u.0 * v.2,
        u.0 * v.1 - u.1 * v.0,
    );
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    return u.0 * v.0 + u.1 * v.1 + u.2 * v.2;
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    let l = v.length();
    return v / l;
}
