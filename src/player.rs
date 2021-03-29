use macroquad::prelude::*;

pub trait Renderable {
    fn render(&self);
}

pub trait Input {
    fn process_input(&mut self);
}

pub struct Player
{
    health: f32,
    pos: Vec2,
    radius: f32,
}

impl Player {
    pub fn new(pos: Vec2, radius: f32) -> Self {
        Self { health: 100.0, pos: pos, radius: radius }
    }
}

impl Renderable for Player {
    fn render(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, YELLOW);
    }
}

impl Input for Player {
    fn process_input(&mut self) {
        if is_key_down(KeyCode::W) {
            self.pos.y -= 5.0;
        }

        if is_key_down(KeyCode::A) {
            self.pos.x -= 5.0;
        }

        if is_key_down(KeyCode::S) {
            self.pos.y += 5.0;
        }

        if is_key_down(KeyCode::D) {
            self.pos.x += 5.0;
        }
    }
}
