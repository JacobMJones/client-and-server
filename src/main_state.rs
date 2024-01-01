use crate::event_handler::EventHandler;
use crate::network_client::NetworkClient;
use crate::other_player::OtherPlayer;
use crate::player::Player;
use ggez::{event, graphics, timer, Context, GameResult};
use gilrs::Gilrs;
use std::collections::HashMap;
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
    other_players: Arc<Mutex<HashMap<String, OtherPlayer>>>,
    focused: bool,
    client_id: String,
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
        let client_id = network_client.client_id.clone(); 
        Ok(MainState {
            event_handler,
            player,
            network_client: Arc::new(Mutex::new(network_client)),
            last_get_time: Instant::now(),
            get_interval: Duration::from_millis(50),
            rt_handle,
            other_players: Arc::new(Mutex::new(HashMap::new())),
            focused:true,
            client_id
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn focus_event(&mut self, _ctx: &mut Context, gained_focus: bool) {
        if gained_focus {
            // The window gained focus
            self.focused = true;
        } else {
            // The window lost focus
            self.focused = false;
        }



    }
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Time elapsed since the last frame
        let dt = timer::delta(ctx).as_secs_f32();
        // println!("Update called with delta time: {}", dt);

        // Process events
        // println!("Processing events...");
        if self.focused {
            self.event_handler.process_events(&mut self.player);
        } else {
            {};
        }
        

        // Update player
        self.player.update(dt);



        let other_players_clone = Arc::clone(&self.other_players);
        let network_client = Arc::clone(&self.network_client);
        let player_position = self.player.position;

        //  println!("Player position to be sent: {:?}", player_position);

        // Spawn async task for setting player position
        tokio::spawn(async move {
            // println!("Starting async task for setting player position...");
            let mut client = network_client.lock().await;
            
            //  println!("Network client locked for setting position...");
            client.set_player_position(player_position).await;
            //  println!("Player position sent to server.");
        });

        // Checking if it's time to fetch updates
        // Checking if it's time to fetch updates
        if Instant::now() - self.last_get_time > self.get_interval {
            self.last_get_time = Instant::now();
        
            // Clone network_client for another async task
            let network_client = Arc::clone(&self.network_client);
        
            let client_id = self.client_id.clone();
            
            // Spawn async task for getting player updates
            self.rt_handle.spawn(async move {
                let mut client = network_client.lock().await;
                match client.get_server_update("PUD").await {
                    Ok(updates) => {
                        println!("client ID {}", client_id);
            
                        // Lock the other_players mutex to safely update it
                        let mut other_players = other_players_clone.lock().await;
            
                        // Iterate through the updates and modify other_players accordingly
                        for (id, (x, y)) in updates {
                            if client_id == id {
                                // Skip this iteration if the id matches the client_id
                                println!("Skipping update for self");
                                continue;
                            }
            
                            // Update or insert the new player data into other_players
                            other_players.entry(id).or_insert_with(|| OtherPlayer::new()).update_position(x, y);
                            // Assuming OtherPlayer has a method like update_position(i32, i32)
                        }
            
                        // println!("Updated other_players: {:?}", other_players);
                    }
                    Err(e) => {
                        // Handle the error, e.g., by logging it
                        eprintln!("Failed to get player update: {}", e);
                    }
                }
            });
        }

        // End of update
        // println!("Update completed.");
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::from_rgb(0, 0, 0));
    
        // Draw the main player (assuming this works and is visible)
        self.player.draw(ctx)?;
    
        // Assuming try_lock is successful and other_players contains correct data
        if let Ok(other_players) = self.other_players.try_lock() {
            for other_player in other_players.values() {

                let circle = graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::fill(),
                    other_player.position,
                    100.0, // Ensure this is a sufficiently large radius
                    1.0, // Circle smoothness
                    graphics::Color::from_rgba(255, 0, 0, 255), // Ensure alpha is 255 for full opacity
                )?;
    
                graphics::draw(ctx, &circle, (other_player.position,))?;
           }
       }
    
        graphics::present(ctx)?;
        Ok(())
    }
    
    
}
