extern crate image;

pub type Pixel = image::Rgb<u8>;
pub type Image = image::RgbImage;

use std::fs::File;
use std::path::Path;

pub const WIDTH: i32 = 800;
pub const HEIGHT: i32 = 800;


pub struct Img {
    pub imgbuf: Image,
}

impl Img {
    pub fn create() -> Img {
        let imgbuf = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
        Img{imgbuf: imgbuf}
    }

    pub fn save(self, path: &'static str) {
        let ref mut fout = File::create(&Path::new(path)).unwrap();
        let _    = image::ImageRgb8(self.imgbuf).save(fout, image::PNG);
    }
    
    pub fn pixel(&mut self, x: i32, y: i32, color: Pixel) {
        if x >=0 && y >=0 && x < WIDTH && y < HEIGHT { 
            self.imgbuf.put_pixel(x as u32, (HEIGHT - y - 1) as u32,color);
        }
    }

    
    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Pixel) {
        if (x1-x0).abs() > (y1-y0).abs() {
            let range = if x1>x0 {x0..x1} else {x1..x0};
            for x in range {
                let t = (x-x0) as f32/(x1-x0) as f32; 
                let y = (y0 as f32 * (1.0-t) + (y1 as f32 *t)) as i32;
                self.pixel(x, y, color);
            }
        } else {
            let range = if y1>y0 {y0..y1} else {y1..y0};
            for y in range {
                let t = (y-y0) as f32/(y1-y0) as f32; 
                let x = (x0 as f32 * (1.0-t) + (x1 as f32 *t)) as i32;
                self.pixel(x, y, color);
            }
        }
    }
    

}

