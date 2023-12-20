
use crate::player::Player;
use ggez::{event, graphics, Context, GameResult};
use gilrs::Gilrs;

use crate::event_handler::EventHandler;

pub struct MainState {
    event_handler: EventHandler,
    player: Player,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        //gamepad
        let gilrs = Gilrs::new().unwrap();
        //gamepad events
        let event_handler = EventHandler::new(gilrs);     
        let player = Player::new();
        Ok(MainState {event_handler, player})
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        // Handle gamepad input
        self.event_handler.process_events(&mut self.player);

        // Update the player
        self.player.update(dt);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(0, 0, 0));

        // Draw the player
        self.player.draw(ctx)?;
        graphics::present(ctx)
    }
}





