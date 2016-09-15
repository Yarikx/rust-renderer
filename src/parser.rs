use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}

struct Face {
    ps: [u32; 3],
}

pub fn parse(filename: &'static str) {   
    let x = File::open(filename)
        .map(|file|  BufReader::new(file) );

    match x {
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
                        vertices.push(Vertex {x: ps[0], y: ps[1], z: ps[2]})
                    },
                    "f" => {
                        let xs = itr
                            .map(|str| -> Vec<u32> {
                                str.split("/")
                                    .filter_map(|s| s.parse::<u32>().ok())
                                    .collect::<Vec<_>>()
                            });
                        let vs = xs.map(|v| v[0]).collect::<Vec<u32>>();
                        faces.push(Face{ps: [vs[0], vs[1], vs[2]]});
                    },
                    _ => {}
                }
            }
            for f in faces {
                println!("{}, {}, {}", f.ps[0], f.ps[1], f.ps[2]);
            }
        },
        Error => println!("no")
    }
        
    //let mut file = BufReader::new(&f);
         
}
