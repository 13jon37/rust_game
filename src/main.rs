use macroquad::prelude::*;

trait Renderable {
    fn render(&self);
}

trait Input {
    fn process_input(&mut self);
}

pub struct Game
{
    screen_width: f32,
    screen_height: f32,
}

struct Player
{
    health: f32,
    pos: Vec2,
    radius: f32,
}

impl Player {
    fn new(pos: Vec2, radius: f32) -> Self {
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

fn game_update_and_render(player: &mut Player, enemy: &Player) {
    player.render();
    player.process_input();
    enemy.render();
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Game".to_owned(),
        fullscreen: false,
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main()
{
    let config: Conf = window_conf();

    // Allocate game struct
    let game: Game = Game {
        screen_width: config.window_width as f32,
        screen_height: config.window_height as f32 };

    let middle: Vec2 = Vec2::new(game.screen_width / 2.0, game.screen_height / 2.0);

    let mut player: Player = Player::new(middle, 15.0);
    let enemy: Player  = Player::new(Vec2::new(250.0, 50.0), 25.0);

    loop {
        clear_background(BLACK);

        draw_text("Bitch ass retard", 20.0, 20.0, 20.0, RED);

        game_update_and_render(&mut player, &enemy);

        next_frame().await;
    }
}
