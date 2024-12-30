use macroquad::prelude::*;

struct Player {
    y: f32,
    width: f32,
    height: f32
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
    fn update(&self) {
        
    }
}


#[macroquad::main("Pong")]
async fn main() {
    request_new_screen_size(800.0, 620.0);

    pub const SPEED: f32 = 3.0;

    let mut player = Player {
        y: 100.,
        width: 30.,
        height: 90.
    };
    let mut ball = Ball{ 
        x: 400.0, 
        y: 310.0,
        velocity: Velocity{x: -0.2, y: 0.0}
    };

    loop {
        clear_background(BLACK);


        draw_rectangle(100.0, player.y, player.width, player.height, WHITE);
        player.update(SPEED);

        draw_circle(ball.x, ball.y, 10.0, WHITE);
        ball.update();
        

        next_frame().await;
    }
}

 