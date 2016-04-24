use piston::input::*;
use opengl_graphics::{ GlGraphics };

pub struct Game {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub rotation: f64,   // Rotation for the square.
    pub x: f64,
    pub y: f64,
    pub is_up_pressed: bool,
    pub is_down_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool
}

impl Game {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        //const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (self.x, self.y);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
        let dy = if self.is_right_pressed { 1.0 } 
            else if self.is_left_pressed { -1.0 } 
            else { 0.0 };
        
        let dx = if self.is_down_pressed { 1.0 } 
            else if self.is_up_pressed { -1.0 }
            else { 0.0 };
        
        self.x += dy * 50.0 * args.dt;
        self.y += dx * 50.0 * args.dt;
    }

    pub fn input(&mut self, inp: Input) {
        match inp {
            Input::Press(but) => {
                match but {
                    Button::Keyboard(Key::Up) => {
                        self.is_up_pressed = true;
                    }
                    Button::Keyboard(Key::Down) => {
                        self.is_down_pressed = true;
                    }
                    Button::Keyboard(Key::Right) => {
                        self.is_right_pressed = true;
                    }
                    Button::Keyboard(Key::Left) => {
                        self.is_left_pressed = true;
                    }
                    _ => {}
                }
            }
            Input::Release(but) => {
                match but {
                    Button::Keyboard(Key::Up) => {
                        self.is_up_pressed = false;
                    }
                    Button::Keyboard(Key::Down) => {
                        self.is_down_pressed = false;
                    }
                    Button::Keyboard(Key::Right) => {
                        self.is_right_pressed = false;
                    }
                    Button::Keyboard(Key::Left) => {
                        self.is_left_pressed = false;
                    }
                    _ => {}
                }
            }
            _ => {
            }
        }
        
    }
}
