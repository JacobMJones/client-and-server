mod player;
mod main_state;
mod other_player;
mod event_handler;
mod network_client;
use ggez::{conf, event, ContextBuilder};
use main_state::MainState;
use tokio::runtime::Runtime;
use crate::network_client::NetworkClient;


fn main() -> ggez::GameResult {
    // Create a new Tokio runtime
    let rt = Runtime::new().unwrap();

    // Keep the runtime handle for later use
    let rt_handle = rt.handle().clone();

    // Use the runtime to initialize and connect the NetworkClient
    let network_client = rt.block_on(async {
        NetworkClient::initialize_and_connect("127.0.0.1:6379").await
    });

    // Build the ggez context and event loop
    let (mut ctx, event_loop) = ContextBuilder::new("top_down_shooter", "author")
        .window_setup(conf::WindowSetup::default().title("Top Down Shooter"))
        .window_mode(conf::WindowMode::default().dimensions(2000.0, 2000.0))
        .build()?;

    // Create the MainState, passing the network_client and runtime handle
    let state = MainState::new(&mut ctx, network_client, rt_handle)?;

    // Run the game loop within the Tokio runtime
    rt.block_on(async {
        event::run(ctx, event_loop, state)
    })
}