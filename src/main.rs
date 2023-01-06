pub mod geometry;
pub mod impls;
pub mod matrix;
pub mod mesh;
pub mod part;
pub mod render;
pub mod sphere;
pub mod utils;
use std::time;
use std::time::Instant;

use crate::matrix::Mat4;
use crate::mesh::Mesh;
use crate::part::*;
use crate::render::*;
use macroquad::prelude::*;

#[macroquad::main("World")]
async fn main() {
    println!("Hello, world!");
    let mut world = World::new_empty();
    world.camera.renderer = Renderer::Simple(RED);
    let mut m_cube = Mesh::cube(1.0);
    m_cube.mesh_color_randomise();
    let cube = Assembly::from_mesh(m_cube);
    world.add_part(cube);
    loop {
        let now = Instant::now();
        let p = macroquad::input::mouse_position_local();
        world.transform_mesh_index(
            0,
            Mat4::rotate_y(p.y / 10.0).mul(&Mat4::rotate_z(p.x / 10.0)),
        );
        println!("elapsed: {:#?}", now.elapsed());
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
