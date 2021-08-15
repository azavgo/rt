use std::fs::write;
use std::io::Error;

use crate::colour::Colour;
use crate::size::Size;

pub fn to_canvas(size: Size) -> Vec<Vec<Colour>> {
    let mut canvas = vec![vec![Colour::new(0, 0, 0); size.w()]; size.h()];
    let mut r = 0;
    let mut g = 0;
    let mut b: u8 = 0;
    for j in 0..size.h() {
        for i in 0..size.w() {
            r = i as u8;
            g = j as u8;
            b = 63;
            canvas[i][j] = Colour::new(r as u8, g as u8, b as u8);
        }
    }
    canvas
}

pub fn to_file(filename: String, canvas: Vec<Vec<Colour>>, size: Size) -> Result<(), Error> {
    let mut ppm_image = format!("P3\n{} {}\n255\n", size.w(), size.h());
    for j in 0..size.w() {
        print!("Rendering image: {}%\r", &j*100/255); 
        for i in 0..size.h() {
            ppm_image = format!(
                "{}{} {} {}\n",
                ppm_image,
                canvas[i][j].r(),
                canvas[i][j].g(),
                canvas[i][j].b()
            );
        }
    }
    write(filename, ppm_image)?;

    Ok(())
}