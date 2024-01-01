use ggez::{graphics, Context, GameResult};
use mint;
pub const MOVEMENT_SPEED: f32 = 1000.0;
pub const CIRCLE_RADIUS: f32 = 45.0;
pub const PLAYER_START_X_POS: f32 = 0.0;
pub const PLAYER_START_Y_POS: f32 = 0.0;

pub struct Player {
    pub position: mint::Point2<f32>,
    pub axis_left: (f32, f32),
    pub speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: mint::Point2 { x: PLAYER_START_X_POS, y: PLAYER_START_Y_POS },
            axis_left: (0.0, 0.0),
            speed: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.update_movement(dt);
    }

    fn update_movement(&mut self, dt: f32) {
        if self.axis_left.0 != 0.0 || self.axis_left.1 != 0.0 {
            self.speed = MOVEMENT_SPEED;
        } else {
            self.speed = 0.0;
        }

        let movement = mint::Vector2 { 
            x: self.axis_left.0 * self.speed * dt, 
            y: self.axis_left.1 * self.speed * dt 
        };

        self.position.x += movement.x;
        self.position.y += movement.y;
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            mint::Point2 { x: 0.0, y: 0.0 },
            CIRCLE_RADIUS,
            0.1, // Smoothness
            graphics::Color::from_rgb(255, 255, 255), // White color
        )?;

        graphics::draw(
            ctx,
            &circle,
            graphics::DrawParam::new()
                .dest(self.position),
        )
    }
}
