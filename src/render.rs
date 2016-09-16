extern crate image;
extern crate nalgebra as na;

pub type Pixel = image::Rgb<u8>;
type Image = image::RgbImage;

pub type Vec2i = na::Vector2<i32>;

use std::fs::File;
use std::path::Path;

use na::Vector2;

pub struct Img {
    width: u32,
    height: u32,
    imgbuf: Image, 
}

trait MyVec {
    fn as_float(self) -> Vector2<f32>;
}

impl MyVec for Vector2<i32> {
    fn as_float(self) -> Vector2<f32> {
        Vector2::new(self.x as f32, self.y as f32)
    }
}

impl Img {
    pub fn create(w: u32, h: u32) -> Img {
        let imgbuf = image::ImageBuffer::new(w, h);
        Img{width: w, height: h, imgbuf: imgbuf}
    }

    pub fn save(self, path: &'static str) {
        let ref mut fout = File::create(&Path::new(path)).unwrap();
        let _    = image::ImageRgb8(self.imgbuf).save(fout, image::PNG);
    }
    
    pub fn pixel(&mut self, x: i32, y: i32, color: Pixel) {
        if x >=0 && y >=0 && x < self.width as i32 && y < self.height as i32{ 
            self.imgbuf.put_pixel(x as u32, (self.height - y as u32 - 1),color);
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

    pub fn triangle(&mut self, t0: Vec2i, t1: Vec2i, t2: Vec2i, color: Pixel) {
        if t0.y == t1.y && t1.y == t2.y {return}
        
        let (t0,t1) = if t0.y>t1.y { (t1, t0)} else { (t0, t1) };
        let (t0,t2) = if t0.y>t2.y { (t2, t0)} else { (t0, t2) };
        let (t1,t2) = if t1.y>t2.y { (t2, t1)} else { (t1, t2) };

        let height = t2.y - t0.y;
        for i in 0..height {
            let second_half = i > t1.y - t0.y || t1.y == t0.y;
            let segment_height =
                if second_half {
                    t2.y - t1.y
                } else {
                    t1.y - t0.y
                } as f32;

            let alpha = i as f32 / height as f32;
            let tmp = if second_half {t1.y-t0.y} else {0};
            let beta = (i - tmp) as f32 / segment_height;

            let a = t0.as_float() + (t2 - t0).as_float() * alpha;
            let b = if second_half {
                t1.as_float() + (t2-t1).as_float() * beta
            } else {
                t0.as_float() + (t1-t0).as_float() * beta
            };

            let (a,b) = if a.x>b.x {(b,a)} else {(a,b)};
            for x in a.x as i32..b.x as i32+1 {
                self.pixel(x,t0.y+i, color);
            }
        }
    }
}

pub fn pixel(r: u8, g: u8, b: u8) -> Pixel {
    image::Rgb([r,g,b])
}

pub fn vec2(x: i32, y: i32) -> Vec2i {
    Vector2 {x: x, y: y}
}

