use super::super::model::vec3;
use super::super::model::ray;
use super::super::util::write;

pub fn gradient() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::Point3(0.0, 0.0, 0.0);
    let horizontal = vec3::Vec3(viewport_width, 0.0, 0.0);
    let vertical = vec3::Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - vec3::Vec3(0.0, 0.0, focal_length);

    // Render

    write::ppm_write_header(image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let u = (i as f64) / (image_width-1) as f64;
            let v = (j as f64) / (image_height-1)  as f64;
            let r = ray::Ray::init(origin, lower_left_corner + horizontal * u + vertical * v - origin);
            let pixel_color = ray::ray_color(&r);
            write::ppm_write_color(pixel_color);
        }
    }
    eprint!("\nDone\n");
}
