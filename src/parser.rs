extern crate nalgebra as na;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::io;

use na::Vector3;

pub struct Face {
    pub ps: [usize; 3],
}

pub struct Model {
    pub vertices: Vec<Vector3<f32>>,
    pub faces: Vec<Face>,
}

pub fn parse(filename: &'static str) -> io::Result<Model> {   
    let x = File::open(filename)
        .map(|file|  BufReader::new(file) );

    return match x {
        Ok(file) => {
            let mut vertices = Vec::new();
            let mut faces = Vec::new();
            for line in file.lines() {
                let l = line.unwrap();
                let mut itr = l.split(" ");
                let command = itr.next().unwrap();
                match command {
                    "v" => {
                        let ps = itr.filter_map(|s| s.parse::<f32>().ok()).collect::<Vec<_>>();
                        vertices.push(Vector3::new(ps[0], ps[1], ps[2]))
                    },
                    "f" => {
                        let xs = itr
                            .map(|str| -> Vec<u32> {
                                str.split("/")
                                    .filter_map(|s| s.parse::<u32>().ok())
                                    .map(|x| x - 1)
                                    .collect::<Vec<_>>()
                            });
                        let vs = xs.map(|v| v[0]).collect::<Vec<u32>>();
                        faces.push(Face{ps: [vs[0] as usize, vs[1] as usize , vs[2] as usize]});
                    },
                    _ => {}
                }
            }
            Ok(Model {vertices: vertices, faces: faces})
        },
        Err(x) => Err(x)
    }   
}
