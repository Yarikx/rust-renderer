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

pub struct Face {
    pub ps: [usize; 3],
    pub vt: [usize; 3],
}

pub struct Model {
    pub vertices: Vec<Vector3<f32>>,
    pub faces: Vec<Face>,
    pub vt: Vec<Vector2<f32>>
}

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub image: RgbaImage,
}

pub fn parse(filename: &'static str) -> io::Result<Model> {   
    let x = File::open(filename)
        .map(|file|  BufReader::new(file) );

    return match x {
        Ok(file) => {
            let mut vertices = Vec::new();
            let mut faces = Vec::new();
            let mut vt = Vec::new();
            for line in file.lines() {
                let l = line.unwrap();
                let mut itr = l.split(" +");
                let command = itr.next().unwrap();
                match command {
                    "v" => {
                        let ps = itr.filter_map(|s| s.parse::<f32>().ok()).collect::<Vec<_>>();
                        vertices.push(Vector3::new(ps[0], ps[1], ps[2]))
                    },
                    "f" => {
                        let xs = itr
                            .map(|str| -> Vec<usize> {
                                str.split("/")
                                    .filter_map(|s| s.parse::<usize>().ok())
                                    .map(|x| x - 1)
                                    .collect::<Vec<_>>()
                            });

                        let mut ps: [usize; 3] = [0,0,0];
                        let mut vt: [usize; 3] = [0,0,0];
                        for (i, x) in xs.enumerate() {
                            ps[i] = x[0];
                            vt[i] = x[1];
                        }
                        faces.push( Face {
                            ps: ps,
                            vt: vt
                        });
                    },
                    "vt" => {
                        let x = itr.next().unwrap().parse::<f32>().unwrap();
                        let y = itr.next().unwrap().parse::<f32>().unwrap();
                        vt.push(Vector2::new(x,y));
                    }
                    _ => {}
                }
            }
            Ok(Model {vertices: vertices, faces: faces, vt: vt})
        },
        Err(x) => Err(x)
    }   
}

pub fn texture() -> ImageResult<Texture> {
    println!("loading texture");
    let reader = BufReader::new(File::open("african_head_diffuse.png").unwrap());
    let mut decoder = PNGDecoder::new(reader);
    let (w,h) = decoder.dimensions().unwrap();
    decoder.into_frames()
        .map(|mut frames| frames.next().unwrap())
        .map(|frame| frame.into_buffer())
        .map(|buffer| {
            Texture {
                width: w,
                height: h,
                image: buffer
            }
        })
}
