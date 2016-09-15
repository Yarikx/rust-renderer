//!An example of generating julia fractals.
extern crate image;

mod parser;
mod render;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;

fn main() {
    let pix = image::Rgb([255, 255, 255]);

    let mut img = render::Img::create();
    
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

    img.save("out.png");

    println!("done!");
}
