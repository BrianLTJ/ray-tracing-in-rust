use super::vec3::unit_vector;
use super::vec3::Color;
use super::vec3::Point3;
use super::vec3::Vec3;

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn init(original: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: Point3(original.0, original.1, original.2),
            dir: Vec3(direction.0, direction.1, direction.2),
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

pub fn ray_color(ray: &Ray) -> Color {
    let unit_direction = unit_vector(ray.direction());
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    let c1 = Color(1.0, 1.0, 1.0) * (1.0 - t);
    let c2 = Color(0.5, 0.7, 1.0) * t;
    c1 + c2
}
