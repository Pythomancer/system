pub mod geometry;
pub mod impls;
pub mod matrix;
pub mod mesh;
pub mod part;
pub mod render;
pub mod sphere;
pub mod utils;
use crate::mesh::Mesh;
use crate::part::*;
use crate::render::*;
use macroquad::prelude::*;

#[macroquad::main("World")]
async fn main() {
    println!("Hello, world!");
    let mut world = World::new_empty();
    let cube = Mesh::cube(1.0);
    world.add_part(Part::from_mesh(cube));
    loop {
        world.render();
        next_frame().await
    }
    // loop {
    //     clear_background(RED);

    //     draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    //     draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    //     draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
    //     draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

    //     next_frame().await
    // }
}
