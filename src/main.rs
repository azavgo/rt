mod vec3;

use std::fs::write;
use vec3::Colour;

fn main() {
    let mut ppm_image = format!("P3\n{} {}\n255\n", 256, 256);

    let mut pixel_colour = Colour::e();

    for j in (0..256).rev() {
        print!("Rendering image: {}%\r", (255 - &j) * 100 / 255);
        for i in 0..256 {
            pixel_colour = Colour::new(i as f64 / 255.0, j as f64 / 255.0, 0.25);

            ppm_image = format!(
                "{}{}\n",
                ppm_image,
                pixel_colour.write_colour()
            );
        }
    }
    write("output.ppm", ppm_image).unwrap();
}
