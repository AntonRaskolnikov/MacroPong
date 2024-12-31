use macroquad::prelude::*;
use crate::structures::{Ball, Entity};

pub mod structures;

const SPEED: f32 = 3.0;

#[macroquad::main("Pong")]
async fn main() {
    request_new_screen_size(800.0, 620.0);

    let mut player = Entity::from(100., 100., 30., 90., true);
    let mut enemy = Entity::from(700., 100., 30., 90., false);
    let mut ball = Ball::from(Rect { x: 400., y: 310., w: 20., h: 20. }, Vec2 { x: -0.5, y: 0.5 });

    loop {
        clear_background(BLACK);

        player.update(&ball);
        enemy.update(&ball);
        ball.update();


        draw_rectangle(ball.rect.x, ball.rect.y, 20., 20., WHITE);
        

        next_frame().await;
    }
}

 