use super::super::model::vec3::Color;

pub fn ppm_write_header(image_width:i32, image_height:i32) {
    print!("P3\n{} {}\n255\n", image_width, image_height);
}

pub fn ppm_write_color(color:Color) {
    let ir = (255.999 * color.x()) as i32;
    let ig = (255.999 * color.y()) as i32;
    let ib = (255.999 * color.z()) as i32;
    print!("{} {} {}\n", ir, ig, ib);
}