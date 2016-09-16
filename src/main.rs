extern crate nalgebra as na;
extern crate rand;

mod parser;
mod render;

use render::pixel;
use render::vec2;

//use na::Vector2;
use na::Vector3;
use na::Cross;
use na::Norm;
use na::Dot;

const WIDTH: u32 = 1800;
const HEIGHT: u32 = 1800;

fn main() {
    let mut img = render::Img::create(WIDTH, HEIGHT);

    match parser::parse("african_head.obj") {
        Ok(model) => {
            let ref vs = model.vertices;
            let w = WIDTH as f32;
            let h = HEIGHT as f32;
            
            for face in &model.faces {
                let mut screen_coords = Vec::new();
                let mut world_coords = Vec::new();
                
                for i in 0..3 {
                    let ref vertex = vs[face.ps[i]];
                    let x = (vertex.x + 1.0) * w / 2.0;
                    let y = (vertex.y + 1.0) * h / 2.0;
                    let vector = vec2(x as i32, y as i32);
                    screen_coords.push(vector);
                    world_coords.push(vertex);
                }

                let x1: Vector3<f32> = world_coords[2] - world_coords[0];
                let x2: Vector3<f32> = world_coords[1] - world_coords[0];
                let n = x1.cross(&x2).normalize();

                let ref light_dir = Vector3::new(0., 0., -1.).normalize();
                let intensity = n.dot(light_dir);

                if intensity > 0. {
                    let br: u8 = (intensity * 255.0) as u8;
                    let color = pixel(br, br, br);
                    img.triangle(screen_coords[0], screen_coords[1], screen_coords[2], color);
                }
            }
        },
        Err(x) => println!("error: {}", x)
    }

    img.save("out.png");

    println!("done!");
}
