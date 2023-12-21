use crate::event_handler::EventHandler;
use crate::network_client::NetworkClient;
use crate::other_player::OtherPlayer;
use crate::player::Player;
use ggez::{event, graphics, timer, Context, GameResult};
use gilrs::Gilrs;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
pub struct MainState {
    event_handler: EventHandler,
    player: Player,
    network_client: Arc<Mutex<NetworkClient>>,
    last_get_time: Instant,
    get_interval: Duration,
    rt_handle: tokio::runtime::Handle,
    other_player: Arc<Mutex<OtherPlayer>>,
}

impl MainState {
    pub fn new(
        ctx: &mut ggez::Context,
        network_client: NetworkClient,
        rt_handle: tokio::runtime::Handle,
    ) -> ggez::GameResult<MainState> {
        let gilrs = Gilrs::new().unwrap();
        let event_handler = EventHandler::new(gilrs);

        let player = Player::new();
        let other_player = Arc::new(Mutex::new(OtherPlayer::new()));
        Ok(MainState {
            event_handler,
            player,
            network_client: Arc::new(Mutex::new(network_client)),
            last_get_time: Instant::now(),
            get_interval: Duration::from_millis(500),
            rt_handle,
            other_player,
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Time elapsed since the last frame
        let dt = timer::delta(ctx).as_secs_f32();
        // println!("Update called with delta time: {}", dt);

        // Process events
        // println!("Processing events...");
        self.event_handler.process_events(&mut self.player);

        // Update player
        // println!("Updating player...");
        self.player.update(dt);

        // Clone network_client for async task
        // Clone the `rt_handle` before the async block.
        let rt_handle = self.rt_handle.clone();

        // If you need to use `other_player` within the async block, clone its Arc.
        let other_player = Arc::clone(&self.other_player);
        let network_client = Arc::clone(&self.network_client);
        let player_position = self.player.position;
        println!("Player position to be sent: {:?}", player_position);

        // Spawn async task for setting player position
        tokio::spawn(async move {
            // println!("Starting async task for setting player position...");
            let mut client = network_client.lock().await;
            //  println!("Network client locked for setting position...");
            client.set_player_position(player_position).await;
            //  println!("Player position sent to server.");
        });

        // Checking if it's time to fetch updates
        if Instant::now() - self.last_get_time > self.get_interval {
            println!("Time to fetch updates from server.");
            self.last_get_time = Instant::now();

            // Clone network_client for another async task
            let network_client = Arc::clone(&self.network_client);

            // Spawn async task for getting player updates
            self.rt_handle.spawn(async move {
                println!("Starting async task for getting player updates...");
                let mut client = network_client.lock().await;
                println!("Network client locked for getting updates...");
                let update_result = client.get_player_update("PUD").await;
                println!("Result of get_player_update: {:?}", update_result);
                match update_result {
                    Ok((ip, x, y)) => {
                        // Destructuring the tuple directly
                        println!("Player update received: IP: {}, X: {}, Y: {}", ip, x, y);
                        let mut op = other_player.lock().await; // Lock the mutex asynchronously and await the result
                        op.update_from_server(x, y);
                    }
                    Err(e) => eprintln!("Failed to get player update: {}", e),
                }
            });
        } else {
            {};
            // println!("Not time to fetch updates yet.");
        }

        // End of update
        // println!("Update completed.");
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(0, 0, 0));
        self.player.draw(ctx)?;
        match self.other_player.try_lock() {
            Ok(guard) => {
                guard.draw(ctx)?;
            },
            Err(_e) => {
            }
        }
        graphics::present(ctx)
    }
}
