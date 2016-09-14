//!An example of generating julia fractals.
extern crate image;

use std::num;
use image::ImageBuffer;
use std::fs::File;
use std::path::Path;

type Pixel = image::Rgb<u8>;
type Image = image::RgbImage;



fn line(img: &mut Image, x0: i32, y0: i32, x1: i32, y1: i32, color: Pixel) {
    for x in x0..x1 {
        let t = (x-x0) as f32/(x1-x0) as f32; 
        let y = (y0 as f32 * (1.0-t) + (y1 as f32 *t)) as i32;
        img.put_pixel(x as u32, y as u32,color);
    } 
}


fn main() {
    let imgx = 800;
    let imgy = 800;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    let pix = image::Rgb([255, 255, 255]);
    line(&mut imgbuf, 100, 100, 800, 400, pix);



    

    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("out.png")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _    = image::ImageRgb8(imgbuf).save(fout, image::PNG);

    println!("done!");
}
