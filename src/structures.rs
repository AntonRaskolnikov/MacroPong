use macroquad::prelude::*;
use crate::SPEED;

pub struct Collision {
    pub points: Vec<(i32, i32)>
}

impl Collision {
    fn from(rect: Rect) -> Self {
        let mut points: Vec<(i32, i32)> = Vec::new();
        
        for point_x in 0..rect.w as i32 {
            for point_y in 0..rect.h as i32 {
                points.push((rect.x as i32 + point_x, rect.y as i32 + point_y));
            }
        }

        Collision {points: points}
    }

    fn update(&mut self, rect: Rect) {
        let mut points: Vec<(i32, i32)> = Vec::new();
        
        for point_x in 0..rect.w as i32 {
            for point_y in 0..rect.h as i32 {
                points.push((rect.x as i32 + point_x, rect.y as i32 + point_y));
            }
        }

        self.points = points;
    }
}
pub struct Entity {
    pub rect: Rect,
    pub is_player: bool,
    pub collision: Collision
}

impl Entity {
    pub fn update(&mut self, ball: &Ball) {
        if self.is_player {
            if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
                self.rect.y -= SPEED;
            }
            if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
                self.rect.y += SPEED;
            }
        } else {
            self.rect.y = ball.rect.y - (self.rect.h / 2.);
        }

        self.rect.y = self.rect.y.clamp(0.0, 500.0);

        self.collision.update(self.rect);
        
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE);

    }

    pub fn from(x: f32, y: f32, w: f32, h: f32, is_player: bool) -> Self {
        let collision = Collision::from(Rect { x: x, y: y, w: w, h: h });

        Entity { 
            rect: Rect { 
                x: x, 
                y: y,
                w: w, 
                h: h 
            },
            is_player: is_player, 
            collision: collision
        }
    }
}

pub struct Ball {
    pub rect: Rect,
    pub velocity: Vec2
}

impl Ball {
    pub fn from(rect: Rect, velocity: Vec2) -> Self {
        Ball { 
            rect: Rect { x: rect.x, y: rect.y, w: rect.w, h: rect.h },
            velocity: Vec2 { 
                x: velocity.x, 
                y: velocity.y 
            } 
        }
    }

    pub fn update(&mut self) {
        self.rect.x += self.velocity.x;
        self.rect.y += self.velocity.y;

    }
}
