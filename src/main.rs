extern crate nalgebra as na;
extern crate rand;

mod parser;
mod render;

use render::pixel;
use render::vec2;
use rand::Rng;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn main() {
    let pix = pixel(255, 255, 255);

    let mut img = render::Img::create(WIDTH, HEIGHT);

    let mut rng = rand::thread_rng();
    //.gen_range(0, 100);
    
    match parser::parse("african_head.obj") {
        Ok(model) => {
            let ref vs = model.vertices;
            let w = WIDTH as f32;
            let h = HEIGHT as f32;
            
            for face in &model.faces {
                let mut vecs = Vec::new();
                for i in 0..3 {
                    let ref vertex = vs[face.ps[i]];
                    let x = (vertex.x + 1.0) * w / 2.0;
                    let y = (vertex.y + 1.0) * h / 2.0;
                    let vector = vec2(x as i32, y as i32);
                    vecs.push(vector);
                }

                let color = pixel(rng.gen(), rng.gen(), rng.gen());
                img.triangle(vecs[0], vecs[1], vecs[2], color);
            }
        },
        Err(x) => println!("error: {}", x)
    }

    img.save("out.png");

    println!("done!");
}
