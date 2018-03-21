extern crate piston_window;
extern crate rand;

use piston_window::*;

fn main() {
    let mut window: piston_window::PistonWindow = piston_window::WindowSettings::new("Space Miner", [960, 540])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|error| {panic!("Build failed: {}", error)});

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0],
                      [0.0, 0.0, 100.0, 100.0],
                      context.transform,
                      graphics);
        });
    }
}