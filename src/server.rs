use mini_redis::Command::{self, Get, Set};
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};

#[derive(Debug)]
struct PlayerData {
    client_ip: String,
    client_position_x: f32,
    client_position_y: f32,
}
type PlayerPositions = Arc<tokio::sync::Mutex<HashMap<String, PlayerData>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening");
    let player_positions: PlayerPositions = Arc::new(tokio::sync::Mutex::new(HashMap::new()));
    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        println!("Accepted from: {}", addr.ip());

        let player_positions_clone = player_positions.clone();
        tokio::spawn(async move {
            process(socket, player_positions_clone, addr.ip().to_string()).await;
        });
    }
}

async fn process(socket: TcpStream, player_positions: PlayerPositions, client_ip: String) {
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let player_id = cmd.key().to_string();
                let value_str = String::from_utf8(cmd.value().to_vec()).unwrap();

                let coords: Vec<&str> = value_str.split(',').collect();
                if coords.len() == 2 {
                    let x = coords[0].parse::<f32>().ok().map(|num| num.round() as i32);
                    let y = coords[1].parse::<f32>().ok().map(|num| num.round() as i32);

                    if let (Some(x), Some(y)) = (x, y) {
                        let mut player_positions = player_positions.lock().await;

                        // Check if the player is new
                        if !player_positions.contains_key(&player_id) {
                            let player_data = PlayerData {
                                client_ip: client_ip.clone(),
                                client_position_x: x as f32,
                                client_position_y: y as f32,
                            };

                            // Print the positions for the new player
                            println!("New Player Joined! Current Player Positions: {:?}", *player_positions);

                            // Store the player data
                            player_positions.insert(player_id.clone(), player_data);
                        } else {
                            // Update the existing player's data
                            if let Some(existing_player_data) = player_positions.get_mut(&player_id) {
                                existing_player_data.client_position_x = x as f32;
                                existing_player_data.client_position_y = y as f32;
                            }
                        }
                        println!("Current Player Positions: {:?}", *player_positions);
                        Frame::Simple("OK".to_string())
                    } else {
                        Frame::Error("Invalid coordinates".into())
                    }
                } else {
                    Frame::Error("Incorrect coordinate format".into())
                }
            }
            Get(cmd) => {
                let all_positions: Vec<String> = player_positions
                    .lock()
                    .await
                    .iter()
                    .map(|(id, player_data)| {
                        format!(
                            "{}:({}, {})",
                            id,
                            player_data.client_position_x,
                            player_data.client_position_y
                        )
                    })
                    .collect();
                let positions_str = all_positions.join("|");
                Frame::Bulk(positions_str.into())
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}
