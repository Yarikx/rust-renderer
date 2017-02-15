extern crate nalgebra as na;
extern crate image;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::io;

use na::Vector3;
use na::Vector2;

use image::png::PNGDecoder;
use image::ImageDecoder;
use image::RgbaImage;
use image::ImageResult;

use image::Pixel;

use utils::both;

/// Triangle plane
pub struct Face {
    /// indexes of coordinate 3d vertices
    pub ps: [usize; 3],
    /// indexes of texture 2d positions
    pub vt: [usize; 3],
}

pub struct Model {
    pub vertices: Vec<Vector3<f32>>,
    pub faces: Vec<Face>,
    pub vt: Vec<Vector2<f32>>
}

/// Represent image in memory with defined with and height
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub image: RgbaImage,
}

impl Texture {
    pub fn get_pixel(&self, x: u32, y: u32) -> image::Rgb<u8> {
        if x >= self.width || y > self.height { return image::Rgb([255, 255, 255]) };
        self.image.get_pixel(x, self.height - y - 1).to_rgb()
    }
}

pub fn parse(filename: &'static str) -> io::Result<Model> {
    File::open(filename)
        .map(|file| {
            let reader = BufReader::new(file);
            let mut vertices = Vec::new();
            let mut faces = Vec::new();
            let mut vt = Vec::new();
            for line in reader.lines() {
                let l = line.unwrap();
                let mut itr = l.split_whitespace();
                let command = itr.next();

                match command {
                    Some("v") => {
                        let ps = itr.filter_map(|s| s.parse::<f32>().ok()).collect::<Vec<_>>();
                        vertices.push(Vector3::new(ps[0], ps[1], ps[2]))
                    },
                    Some("f") => {
                        let xs = itr
                            .map(|str| -> Vec<usize> {
                                str.split("/")
                                    .filter_map(|s| s.parse::<usize>().ok())
                                    .map(|x| x - 1)
                                    .collect::<Vec<_>>()
                            });

                        let mut ps: [usize; 3] = [0, 0, 0];
                        let mut vt: [usize; 3] = [0, 0, 0];
                        for (i, x) in xs.enumerate() {
                            ps[i] = x[0];
                            vt[i] = x[1];
                        }
                        faces.push(Face {
                            ps: ps,
                            vt: vt
                        });
                    },
                    Some("vt") => {
                        let x = itr.next().unwrap().parse::<f32>().unwrap();
                        let y = itr.next().unwrap().parse::<f32>().unwrap();
                        vt.push(Vector2::new(x, y));
                    }
                    _ => {}
                }
            }
            Model { vertices: vertices, faces: faces, vt: vt }
        })
}

pub fn texture(filename: &'static str) -> ImageResult<Texture> {
    println!("loading texture");
    File::open(filename)
        .map(|file| BufReader::new(file))
        .map(|reader| PNGDecoder::new(reader))
        .map_err(|e| image::ImageError::from(e))
        .and_then(|mut decoder| {
            let dimenstions = decoder.dimensions();
            let frame = decoder.into_frames()
                .and_then(|mut frames| {
                    frames.next()
                        .ok_or(image::ImageError::from(io::Error::new(io::ErrorKind::NotFound, "no frames found for texture")))
                });
            both(dimenstions, frame)
        })
        .map(|(dimensions, frame)| {
            let (w, h) = dimensions;
            let buffer = frame.into_buffer();

            println!("texture loaded");
            Texture {
                width: w,
                height: h,
                image: buffer
            }
        })
}

