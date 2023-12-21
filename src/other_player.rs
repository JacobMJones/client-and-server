use ggez::{graphics, Context, GameResult};
use mint;

pub struct OtherPlayer {
    pub position: mint::Point2<f32>,
    pub speed: f32,
}

impl OtherPlayer {
    pub fn new() -> Self {
        OtherPlayer {
            position: mint::Point2 { x: 0.0, y: 0.0 }, // Initial position can be (0,0) or any default value
            speed: 0.0, // Initial speed can be set to 0
        }
    }

    // Update the position based on server messages
    pub fn update_from_server(&mut self, x: i32, y: i32) {
        self.position.x = x as f32 + 200.0;
        self.position.y = y as f32 + 500.0;
        // You might want to adjust speed based on the difference in position or other server data
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            mint::Point2 { x: 0.0, y: 0.0 },
            100.0, // Assuming the same radius as Player
            0.1, // Smoothness
            graphics::Color::from_rgb(255, 0, 0), // Different color for distinction, here red
        )?;

        graphics::draw(
            ctx,
            &circle,
            graphics::DrawParam::new()
                .dest(self.position),
        )
    }
}
