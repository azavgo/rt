use std::io::Error;

mod size;
mod utils; 
mod colour;


use size::Size;

fn main() -> Result<(), Error> {
    let size = Size::new(256, 256);
    let canvas = utils::to_canvas(size);
    utils::to_file("image.ppm".to_string(), canvas, size)?;

    Ok(())
}
