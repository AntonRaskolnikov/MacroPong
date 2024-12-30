use macroquad::prelude::*;

struct Player {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    collision: Vec<(f32, f32)>
}

impl Player {
    fn update(&mut self, speed: f32) {
        if (is_key_down(KeyCode::W) || is_key_down(KeyCode::Up)) && self.y >= 0.0  {
            self.y -= speed;
        }
        if (is_key_down(KeyCode::S) || is_key_down(KeyCode::Down))  && self.y <= 500.0 {
            self.y += speed;
        }
        self.y = self.y.clamp(0.0, 500.0);

        let mut collision: Vec<(f32, f32)> = Vec::new();

        for point_x in 0..self.width as i32 {
            for point_y in 0..self.height as i32 {
                collision.push((self.x + point_x as f32, self.y + point_y as f32));
            }
        }

        self.collision = collision;

    }
    fn from(x: f32, y: f32, width: f32, height: f32) -> Self {
        let mut collision: Vec<(f32, f32)> = Vec::new();

        for point_x in 0..width as i32 {
            for point_y in 0..height as i32 {
                collision.push((x + point_x as f32, y + point_y as f32));
            }
        }

        Player { x: x, y: y, width: width, height: height, collision: collision }
    }
}


struct Velocity {
    x: f32,
    y: f32
}

struct Ball {
    x: f32,
    y: f32,
    velocity: Velocity
}

impl Ball {
    fn update(&mut self, player: &Player) {
        self.x += self.velocity.x;
        self.y += self.velocity.y;
        for i in 0..player.collision.len() {

        }
    }
}


#[macroquad::main("Pong")]
async fn main() {
    request_new_screen_size(800.0, 620.0);

    pub const SPEED: f32 = 3.0;

    let mut player = Player::from(100., 100., 30., 90.);
    let mut ball = Ball{ 
        x: 400.0, 
        y: 310.0,
        velocity: Velocity{x: -0.2, y: 0.0}
    };

    loop {
        clear_background(BLACK);


        draw_rectangle(100.0, player.y, player.width, player.height, WHITE);
        player.update(SPEED);

        draw_rectangle(ball.x, ball.y, 20., 20., WHITE);
        ball.update(&player);
        

        next_frame().await;
    }
}

 