use bytes::Bytes;
use ggez::mint::Point2;
// use local_ip_address::local_ip;
use mini_redis::client;
use mini_redis::client::Client;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use std::collections::HashMap;
pub struct NetworkClient {
    server_address: String,
    pub client_id: String,
    client: Arc<Mutex<Option<Client>>>,
}

impl NetworkClient {
    pub async fn initialize_and_connect(server_address: &str) -> Self {
        // println!("Initializing NetworkClient...");
        let client = NetworkClient::initialize(server_address);
        match client.connect().await {
            Ok(()) => println!("Successfully connected to the server."),
            Err(e) => eprintln!("Failed to connect to the server: {}", e),
        }
        client
    }

    pub fn initialize(server_address: &str) -> Self {
        // println!("Creating NetworkClient instance...");
        let client_id = Uuid::new_v4().to_string();
        println!("client id {}", client_id);
        NetworkClient {
            server_address: server_address.to_string(),
            client_id,
            client: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn connect(&self) -> Result<(), String> {
        println!(
            "Attempting to connect to server at {}...",
            self.server_address
        );
        let mut locked_client = self.client.lock().await;
        *locked_client = match client::connect(&self.server_address).await {
            Ok(client) => Some(client),
            Err(e) => return Err(format!("Connection error: {}", e)),
        };
        Ok(())
    }

    pub async fn set_player_position(&mut self, position: Point2<f32>) {
        let key = format!("{}", self.client_id);
        let position_string = format!("{},{}", position.x, position.y);
        let position_bytes = Bytes::from(position_string.clone()); 
        
      //  println!("Sending to server: key = '{}', position = '{}'", key, position_string);
        
        let mut client_guard = self.client.lock().await;
        if let Some(client) = client_guard.as_mut() {
            match client.set(&key, position_bytes).await {
                Ok(_) => {},
                Err(e) => eprintln!("Failed to set player position: {:?}", e),
            }
        } else {
          //  eprintln!("Client not connected, unable to set player position.");
        }
    }
    // Updated get_player_update method
    pub async fn get_server_update(&mut self, command: &str) -> Result<HashMap<String, (i32, i32)>, String> {
        let mut client_guard = self.client.lock().await;
    
        if let Some(client) = client_guard.as_mut() {
            match client.get(command).await {
                Ok(Some(response)) => {
                    let response_str = String::from_utf8(response.to_vec())
                        .map_err(|e| format!("Invalid UTF-8 sequence: {}", e))?;
    
                    // Each player's data is separated by '|', split the string
                    let players_data: Vec<&str> = response_str.split('|').collect();
                    let mut positions = HashMap::new();
    
                    for data in players_data {
                    
                        let parts: Vec<&str> = data.split(':').collect();
                    
                        if parts.len() == 2 {
                            let player_id = parts[0];
                            let coords: Vec<&str> = parts[1].split(',').collect();
                           
                    
                            if coords.len() == 2 {
                                // Clean and parse X coordinate
                                let x_clean = coords[0].trim().replace("(", "");
                                let x = x_clean
                                    .parse::<f32>()
                                    .map_err(|_| format!("Invalid X coordinate for player {}", player_id))?
                                    .round() as i32;
                    
                                // Clean and parse Y coordinate
                                let y_clean = coords[1].trim().replace(")", "");
                                let y = y_clean
                                    .parse::<f32>()
                                    .map_err(|_| format!("Invalid Y coordinate for player {}", player_id))?
                                    .round() as i32;
                    
                                positions.insert(player_id.to_string(), (x, y));
                            } else {
                                return Err(format!("Invalid coordinate format for player {}", player_id));
                            }
                        } else {
                            return Err("Invalid player data format.".to_string());
                        }
                    }
    
                    Ok(positions)
                }
                Ok(None) => Err("No data received from server.".to_string()),
                Err(e) => Err(format!("Error retrieving data from server: {}", e)),
            }
        } else {
            Err("Client not connected, unable to get player updates.".to_string())
        }
    }
}
