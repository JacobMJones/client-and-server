use crate::collidable::Collidable;
use crate::flash_effect::FlashEffect;
use ggez::graphics::{self, Color, Mesh, Rect};
use ggez::{Context, GameResult};
use mint::Point2;
use rand::Rng;
pub struct Collectible {
    pub position: Point2<f32>,
    pub size: f32,
    pub active: bool,
    pub radius: f32,
    pub time: f32,
    pub id: String,
    pub in_proximity: bool,
    mesh: Mesh,
}

impl Collectible {
    pub fn new(ctx: &mut Context, x: f32, y: f32, size: f32, initial_time: f32, id: String) -> GameResult<Self> {
        let mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(0.0, 0.0, size, size),
            Color::WHITE,
        )?;

        Ok(Collectible {
            position: Point2 { x, y },
            size,
            active: true,
            radius: size / 2.0,
            time: initial_time,
            id,
            in_proximity: false,
            mesh,
        })
    }
    // pub fn new(x: f32, y: f32, size: f32, initial_time: f32, id: String) -> Self {
    //     Collectible {
    //         position: Point2 { x, y },
    //         size,
    //         active: true,
    //         radius: size / 2.0,
    //         time: initial_time,
    //         id,
    //         in_proximity: false,
    //     }
    // }
    fn get_pulsating_size(&self) -> f32 {
        let pulsation_factor = 0.9; // Adjust this value for more/less pulsation
        let min_size = 10.0; // Minimum size
        let max_size = self.size; // Maximum size, based on initial size
        let mut pulsating_size = self.size + pulsation_factor * self.time.sin();

        // Ensure the size is within the min and max bounds
        if pulsating_size < min_size {
            pulsating_size = min_size + (min_size - pulsating_size);
        } else if pulsating_size > max_size {
            pulsating_size = max_size - (pulsating_size - max_size);
        }
        pulsating_size
    }
    fn get_dynamic_color(&self) -> Color {
        if !self.in_proximity {
            // Original dynamic color
            let r = (self.time.sin() * 0.5 + 0.5) as f32;
            let g = ((self.time + 2.0).sin() * 0.5 + 0.5) as f32;
            let b = ((self.time + 4.0).sin() * 0.5 + 0.5) as f32;
            Color::new(r, g, b, 1.0)
        } else {
            // Gray color when in proximity
            let r = (self.time.sin() * 0.5 + 0.5) as f32;
            let g = ((self.time + 2.0).sin() * 0.5 + 0.5) as f32;
            let b = ((self.time + 4.0).sin() * 0.5 + 0.5) as f32;
            Color::new(r, g, b, 0.1)
        }
    }
    // Call this method to mark the collectible as in proximity
    pub fn set_in_proximity(&mut self, in_proximity: bool) {
        self.in_proximity = in_proximity;
    }
    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        if self.active {
            let size = self.get_pulsating_size();
            let color = self.get_dynamic_color();

            // Calculate the scale factor based on the current size
            let scale_x = size / self.size;
            let scale_y = size / self.size;

            // Draw the mesh with updated scale and color
            graphics::draw(
                ctx,
                &self.mesh,
                graphics::DrawParam::default()
                    .dest([self.position.x, self.position.y])
                    .scale([scale_x, scale_y])
                    .color(color),
            )?;
        }
        Ok(())
    }
    pub fn bounding_box(&self) -> Rect {
        Rect::new(
            self.position.x - self.size / 2.0,
            self.position.y - self.size / 2.0,
            self.size,
            self.size,
        )
    }
    pub fn activate_flash_effect(&self, flash_effect_pool: &mut Vec<FlashEffect>) {
        let mut rng = rand::thread_rng(); // Create a random number generator
        for _ in 0..4 {
            // Loop to activate up to 4 effects
            if let Some(inactive_effect) = flash_effect_pool.iter_mut().find(|e| !e.is_active()) {
                // Calculate the base adjusted position
                let base_adjusted_position = Point2 {
                    x: self.position.x + self.size / 2.0,
                    y: self.position.y + self.size / 2.0,
                };

                // Create a random offset
                let offset_x: f32 = rng.gen_range(-10.0..10.0); // Random offset in x direction
                let offset_y: f32 = rng.gen_range(-10.0..10.0); // Random offset in y direction

                // Apply the offset to the base position
                let random_adjusted_position = Point2 {
                    x: base_adjusted_position.x + offset_x,
                    y: base_adjusted_position.y + offset_y,
                };
                let random_duration: f32 = rng.gen_range(0.1..0.4);
                // Activate the effect with the randomly adjusted position
                inactive_effect.activate(
                    random_adjusted_position,
                    Color::new(1.0, 1.0, 1.0, 1.0), // White color
                    random_duration,                // Duration
                );
            }
        }
    }
}

impl Collidable for Collectible {
    fn bounding_box(&self) -> Rect {
        self.bounding_box()
    }
}
