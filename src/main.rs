//!An example of generating julia fractals.
extern crate image;

use std::num;
use std::fs::File;
use std::path::Path;

mod parser;

type Pixel = image::Rgb<u8>;
type Image = image::RgbImage;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;


fn pixel(img: &mut Image, x: i32, y: i32, color: Pixel) {
    if x >=0 && y >=0 && x < WIDTH && y < HEIGHT { 
        img.put_pixel(x as u32, y as u32,color);
    }
}

fn line(img: &mut Image, x0: i32, y0: i32, x1: i32, y1: i32, color: Pixel) {
    if (x1-x0).abs() > (y1-y0).abs() {
        let range = if x1>x0 {x0..x1} else {x1..x0};
        for x in range {
            let t = (x-x0) as f32/(x1-x0) as f32; 
            let y = (y0 as f32 * (1.0-t) + (y1 as f32 *t)) as i32;
            pixel(img, x, y, color);
        }
    } else {
        let range = if y1>y0 {y0..y1} else {y1..y0};
        for y in range {
            let t = (y-y0) as f32/(y1-y0) as f32; 
            let x = (x0 as f32 * (1.0-t) + (x1 as f32 *t)) as i32;
            pixel(img, x, y, color);
        }
    }
}


fn main() {
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    let pix = image::Rgb([255, 255, 255]);
    line(&mut imgbuf, 100, 100, 200, 900, pix);
    
    line(&mut imgbuf, 800, 100, 0, 0, pix);



    

    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("out.png")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _    = image::ImageRgb8(imgbuf).save(fout, image::PNG);

    println!("done!");
    match parser::parse("african_head.obj") {
        Ok(model) => {
            let fs = model.faces;
            let vs = model.vertices;
            let w = WIDTH as f32;
            let h = HEIGHT as f32;
            for face in model.faces {
                for i in 0..3 {
                    let v1 = vs[fs[i]];
                    let v2 = vs[fs[(i+1) % 3]];
                    
                    let x0 = (v0.x + 1.0) * w / 2.0;
                    let y0 = (v0.y + 1.0) * h / 2.0;
                    let x1 = (v1.x + 1.0) * w / 2.0;
                    let y1 = (v1.y + 1.0) * h / 2.0;

                    line(img, x0 as i32, y0 as i32, x1 as i32, y1 as i32, pix);
                }
            }
        },
        Err(x) => println!("error: {}", x)
    }
}
