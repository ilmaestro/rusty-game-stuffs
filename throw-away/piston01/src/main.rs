extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

mod game;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [600, 600]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut game = game::Game {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        x: 0.0,
        y: 0.0,
        is_up_pressed: false,
        is_down_pressed: false,
        is_left_pressed: false,
        is_right_pressed: false
    };

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        match e {
            Event::Update(u) => {
                game.update(&u);
            }
            Event::Render(r) => {
                game.render(&r);
            }
            Event::Input(i) => {
                game.input(i);
            }
            _ => {
            }
        }
    }
}