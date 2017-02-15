extern crate nalgebra as na;
extern crate rand;
extern crate image;

mod parser;
mod render;

use na::Vector2;
use na::Vector3;
use na::Cross;
use na::Norm;
use na::Dot;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const DEPTH: u32 = 250;

fn main() {
    let mut img = render::Img::create(WIDTH, HEIGHT);

    match parser::parse("african_head.obj") {
        Ok(model) => {
            let ref vs = model.vertices;
            let ref vt = model.vt;
            let w = WIDTH as f32;
            let h = HEIGHT as f32;
            let d = DEPTH as f32;

            let texture = parser::texture("african_head_diffuse.png").unwrap();

            for face in &model.faces {
                let mut screen_coords = Vec::new();
                let mut world_coords = Vec::new();
                let mut vt_coords = Vec::new();

                for i in 0..3 {
                    let ref vertex = vs[face.ps[i]];
                    let x = (vertex.x + 1.0) * w / 2.0;
                    let y = (vertex.y + 1.0) * h / 2.0;
                    let z = (vertex.z + 1.0) * d / 2.0;
                    let vector = Vector3::new(x as i32, y as i32, z as i32);
                    screen_coords.push(vector);
                    world_coords.push(vertex);

                    let vt_pos = vt[face.vt[i]];
                    let texture_x = (vt_pos.x * texture.width as f32) as u32;

                    let texture_y = (vt_pos.y * texture.height as f32) as u32;
                    vt_coords.push(Vector2::new(texture_x, texture_y));
                }

                let x1: Vector3<f32> = world_coords[2] - world_coords[0];
                let x2: Vector3<f32> = world_coords[1] - world_coords[0];
                let n = x1.cross(&x2).normalize();

                let ref light_dir = Vector3::new(-0., 0., -1.).normalize();
                let intensity = n.dot(light_dir);

                if intensity > 0. {
                    img.triangle(screen_coords[0], screen_coords[1], screen_coords[2],
                                 vt_coords[0], vt_coords[1], vt_coords[2],
                                 &texture, intensity);
                }
            }
        },
        Err(x) => println!("error: {}", x)
    }

    match img.save("out.png") {
        Ok(_) => println!("done!"),
        Err(x) => println!("error: {}", x)
    }
}
