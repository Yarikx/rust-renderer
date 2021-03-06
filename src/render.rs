extern crate image;
extern crate nalgebra as na;

pub type Pixel = image::Rgb<u8>;
pub type Image = image::RgbImage;

use image::Pixel as IPixel;

pub type Vec2u = na::Vector2<u32>;
pub type Vec3i = na::Vector3<i32>;

use std::fs::File;
use std::path::Path;

use na::Vector2;
use na::Vector3;

use parser::Texture;
use std::mem::swap;

pub struct Img {
    width: u32,
    height: u32,
    imgbuf: Image,
    zbuf: image::GrayImage,
}

trait MyVec {
    fn as_float(&self) -> Vector3<f32>;
}

impl MyVec for Vector3<i32> {
    fn as_float(&self) -> Vector3<f32> {
        Vector3::new(self.x as f32, self.y as f32, self.z as f32)
    }
}

fn as_float(v: Vec2u) -> Vector2<f32> {
    Vector2::new(v.x as f32, v.y as f32)
}

impl Img {
    pub fn create(w: u32, h: u32) -> Img {
        let imgbuf = image::ImageBuffer::new(w, h);
        let zbuf = image::ImageBuffer::new(w, h);
        Img { width: w, height: h, imgbuf: imgbuf, zbuf: zbuf }
    }

    pub fn save(&self, path: &'static str) -> image::ImageResult<()> {
        let buf = image::imageops::rotate180(&self.imgbuf);
        File::create(&Path::new(path))
            .map_err(|e| image::ImageError::from(e))
            .and_then(|ref mut file| image::ImageRgb8(buf).save(file, image::PNG))
    }

    pub fn save_zbuf(&self, path: &'static str) -> image::ImageResult<()> {
        let buf = image::imageops::rotate180(&self.zbuf);
        File::create(&Path::new(path))
            .map_err(|e| image::ImageError::from(e))
            .and_then(|ref mut file| image::ImageLuma8(buf).save(file, image::PNG))
    }

    pub fn pixel(&mut self, x: i32, y: i32, color: Pixel) {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            self.imgbuf.put_pixel(x as u32, y as u32, color);
        }
    }

    #[allow(dead_code)]
    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Pixel) {
        if (x1 - x0).abs() > (y1 - y0).abs() {
            let range = if x1 > x0 { x0..x1 } else { x1..x0 };
            for x in range {
                let t = (x - x0) as f32 / (x1 - x0) as f32;
                let y = (y0 as f32 * (1.0 - t) + (y1 as f32 * t)) as i32;
                self.pixel(x, y, color);
            }
        } else {
            let range = if y1 > y0 { y0..y1 } else { y1..y0 };
            for y in range {
                let t = (y - y0) as f32 / (y1 - y0) as f32;
                let x = (x0 as f32 * (1.0 - t) + (x1 as f32 * t)) as i32;
                self.pixel(x, y, color);
            }
        }
    }

    pub fn triangle(&mut self, t0: Vec3i, t1: Vec3i, t2: Vec3i,
                    uv0: Vec2u, uv1: Vec2u, uv2: Vec2u,
                    texture: &Texture, intensity: f32) {
        if t0.y == t1.y && t1.y == t2.y { return; }

        let (mut t0, mut t1, mut t2, mut uv0, mut uv1, mut uv2) = (t0, t1, t2, uv0, uv1, uv2);
        if t0.y > t1.y {
            swap(&mut t0, &mut t1);
            swap(&mut uv1, &mut uv0);
        }
        if t0.y > t2.y {
            swap(&mut t0, &mut t2);
            swap(&mut uv0, &mut uv2);
        }
        if t1.y > t2.y {
            swap(&mut t1, &mut t2);
            swap(&mut uv1, &mut uv2);
        }

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
            let tmp = if second_half { t1.y - t0.y } else { 0 };
            let beta = (i - tmp) as f32 / segment_height;

            let a = t0.as_float() + (t2 - t0).as_float() * alpha;
            let b = if second_half {
                t1.as_float() + (t2 - t1).as_float() * beta
            } else {
                t0.as_float() + (t1 - t0).as_float() * beta
            };

            let (uv0, uv1, uv2) = (as_float(uv0), as_float(uv1), as_float(uv2));
            let uv_a = uv0 + (uv2 - uv0) * alpha;
            let uv_b = if second_half {
                uv1 + (uv2 - uv1) * beta
            } else {
                uv0 + (uv1 - uv0) * beta
            };

            let (a, b, uv_a, uv_b) = if a.x > b.x {
                (b, a, uv_b, uv_a)
            } else {
                (a, b, uv_a, uv_b)
            };
            for x in a.x as i32..b.x as i32 + 1 {
                let phi = if b.x == a.x {
                    1.
                } else {
                    (x as f32 - a.x) / (b.x - a.x)
                };
                let p = a + ((b - a) * phi);
                let p = Vector3::new(p.x as i32, p.y as i32, p.z as i32);

                let uv = uv_a + ((uv_b - uv_a) * phi);
                let z = self.zbuf.get_pixel(p.x as u32, p.y as u32).channels()[0];
                let pz = p.z as u8;
                if z < pz {
                    self.zbuf.put_pixel(p.x as u32, p.y as u32, image::Luma([pz]));
                    let uv_pixel = texture.get_pixel(uv.x as u32, uv.y as u32)
                        .map(|c| (c as f32 * intensity) as u8);
                    self.pixel(x, t0.y + i, uv_pixel);
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn pixel(r: u8, g: u8, b: u8) -> Pixel {
    image::Rgb([r, g, b])
}

