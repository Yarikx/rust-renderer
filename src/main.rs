extern crate nalgebra as na;

mod parser;
mod render;

use render::pixel;
use render::vec2;

const WIDTH: u32 = 1800;
const HEIGHT: u32 = 1800;

fn main() {
    let pix = pixel(255, 255, 255);

    let mut img = render::Img::create(WIDTH, HEIGHT);
    
    match parser::parse("african_head.obj") {
        Ok(model) => {
            let ref fs = model.faces;
            let ref vs = model.vertices;
            let w = WIDTH as f32;
            let h = HEIGHT as f32;
            for face in fs {
                for i in 0..3 {
                    let v0 = &vs[face.ps[i]];
                    let v1 = &vs[face.ps[(i+1) % 3]];
                    
                    let x0 = (v0.x + 1.0) * w / 2.0;
                    let y0 = (v0.y + 1.0) * h / 2.0;
                    let x1 = (v1.x + 1.0) * w / 2.0;
                    let y1 = (v1.y + 1.0) * h / 2.0;

                    img.line(x0 as i32, y0 as i32, x1 as i32, y1 as i32, pix);
                }
            }
        },
        Err(x) => println!("error: {}", x)
    }

    img.triangle(vec2(0, 0), vec2(100, 200), vec2(200, 100), pix);

    img.save("out.png");

    println!("done!");
}
