use ggez::{graphics, Context, GameResult};
use mint;

#[derive(Debug)]
pub struct OtherPlayer {
    pub position: mint::Point2<f32>,
    pub speed: f32,
}

impl OtherPlayer {
    pub fn new() -> Self {
        OtherPlayer {
            position: mint::Point2 { x: 0.0, y: 0.0 }, // Initial position using players start pos for now
            speed: 0.0, 
        }
    }

    // Update the position based on server messages
    pub fn update_position(&mut self, x: i32, y: i32) {
 
        self.position.x = x as f32;
        self.position.y = y as f32;
        //adjust speed based on the difference in position or other server data?? is this why position is off
        
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {

 
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            mint::Point2 { x: 0.0, y: 0.0 },
            100.0, 
            0.1, 
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
