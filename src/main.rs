extern crate piston_window;
extern crate rand;

mod stage;

use piston_window::*;
use stage::default_generate_stage;

fn main() {
    // let mut window: PistonWindow = PistonWindow::new(
    //     OpenGL::V3_3,
    //     0,
    //     WindowSettings::new("Space Miner", [960, 540])
    //     .opengl(OpenGL::V3_3)
    //     .srgb(false)
    //     .exit_on_esc(true)
    //     .build()
    //     .unwrap_or_else(|error| {panic!("Build failed: {}", error)}),
    // );

    // while let Some(event) = window.next() {
    //     window.draw_2d(&event, |context, graphics| {
    //         clear([1.0; 4], graphics);
    //         rectangle([1.0, 0.0, 0.0, 1.0],
    //                   [0.0, 0.0, 100.0, 100.0],
    //                   context.transform,
    //                   graphics);
    //     });
    // }
    let stage = default_generate_stage();
}