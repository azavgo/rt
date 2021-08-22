mod vec3; 

use std::fs::write; 

fn main() {
    let mut ppm_image = format!("P3\n{} {}\n255\n", 256, 256);
    
    let mut r: f64 = 0.0; 
    let mut g: f64 = 0.0;
    let mut b: f64 = 0.0;
    
    for j in (0..256).rev() {
        print!("Rendering image: {}%\r", (255 - &j) * 100 / 255);
        for i in 0..256 {
            r = i as f64/255.0; 
            g = j as f64/255.0;
            b = 0.25; 
            
            ppm_image = format!(
                "{}{} {} {}\n",
                ppm_image,
                (255.99 * r) as u8,
                (255.99 * g) as u8,
                (255.99 * b) as u8
            );
        }
    }
    write("output.ppm", ppm_image).unwrap();
}