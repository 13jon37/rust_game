use macroquad::prelude::*;
use std::{thread, time};

mod player;

use crate::{player::{Player, Renderable, Input}};

pub struct Game
{
    screen_width: f32,
    screen_height: f32,
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
        screen_height: config.window_height as f32, };

    let middle: Vec2 = Vec2::new(game.screen_width / 2.0, game.screen_height / 2.0);

    let mut player: Player = Player::new(middle, 15.0);
    let enemy: Player  = Player::new(Vec2::new(250.0, 50.0), 25.0);

    loop {
        clear_background(BLACK);

        // Render fps
        let fps: &str = &get_fps().to_string();
        draw_text(fps, 20.0, 20.0, 20.0, RED);

        game_update_and_render(&mut player, &enemy);

        // Using standard library function for now, but I should replace
        // this later with a smoother alternative.
        thread::sleep(time::Duration::from_millis(15));
        next_frame().await;
    }
}
