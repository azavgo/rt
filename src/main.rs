mod vec3;

use std::fs::write;
use vec3::Colour;

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

fn main() {
    let mut ppm_image = format!("P3\n{} {}\n255\n", 256, 256);

    let mut pixel_colour = Colour::e();

    for j in (0..IMAGE_HEIGHT).rev() {
        print!("Rendering image: {}%\r", (IMAGE_HEIGHT - 1 - &j) * 100 / (IMAGE_HEIGHT - 1));
        for i in 0..IMAGE_WIDTH {
            pixel_colour = Colour::new(i as f64 / (IMAGE_WIDTH-1) as f64, j as f64 / (IMAGE_HEIGHT-1) as f64, 0.25);

            ppm_image = format!("{}{}\n", ppm_image, pixel_colour.write_colour());
        }
    }
    write("output.ppm", ppm_image).unwrap();
}
