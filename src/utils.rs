use std::fs::write;
use std::io::Error;

use crate::colour::Colour;
use crate::size::Size;


pub fn to_canvas(size: Size) -> Vec<Vec<Colour>> {
    let mut canvas = vec![vec![Colour::new(0, 0, 0); size.w()]; size.h()];
    //normalised red 0..1, nr; normalised green, ng; normalised blue, nb
    //allows to colour an image of any size, not just 256 x 256
    let mut nr = 0.0; 
    let mut ng = 0.0;
    let mut nb = 0.25;
    for j in (0..size.h()).rev() {
        for i in 0..size.w() {
            nr = (i as f32) / (size.w() as f32);
            ng = (j as f32) / (size.h() as f32);
            canvas[i][j] = Colour::new(
                (nr * 255.999) as u8,
                (ng * 255.999) as u8,
                (nb * 255.999) as u8,
            );
        }
    }
    canvas
}




pub fn to_file(filename: String, canvas: Vec<Vec<Colour>>, size: Size) -> Result<(), Error> {
    let mut ppm_image = format!("P3\n{} {}\n255\n", size.w(), size.h());
    for j in 0..size.w() {
        print!("Rendering image: {}%\r", &j * 100 / 255);
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
