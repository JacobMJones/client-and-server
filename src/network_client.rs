use bytes::Bytes;
use ggez::mint::Point2;
use local_ip_address::local_ip;
use mini_redis::client;
use mini_redis::client::Client;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct NetworkClient {
    server_address: String,
    client: Arc<Mutex<Option<Client>>>,
}

impl NetworkClient {
    pub async fn initialize_and_connect(server_address: &str) -> Self {
        println!("Initializing NetworkClient...");
        let client = NetworkClient::initialize(server_address);
        match client.connect().await {
            Ok(()) => println!("Successfully connected to the server."),
            Err(e) => eprintln!("Failed to connect to the server: {}", e),
        }
        client
    }

    pub fn initialize(server_address: &str) -> Self {
        println!("Creating NetworkClient instance...");
        NetworkClient {
            server_address: server_address.to_string(),
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
        // println!("Setting player position to: {:?}", position);
        let formatted_ip = match local_ip() {
            Ok(ip) => ip.to_string(),
            Err(e) => {
                eprintln!("Failed to get local IP: {}", e);
                return;
            }
        };

        let position_string = format!("{},{}", position.x, position.y);
        let position_bytes = Bytes::from(position_string);

        let mut client_guard = self.client.lock().await;
        match client_guard.as_mut() {
            Some(client) => {
                if let Err(e) = client.set(&formatted_ip, position_bytes).await {
                    eprintln!("Failed to set player position: {}", e);
                }
            }
            None => eprintln!("Client not connected, unable to set player position."),
        }
    }

    // Updated get_player_update method
    pub async fn get_player_update(&mut self, command: &str) -> Result<(String, i32, i32), String> {
        let mut client_guard = self.client.lock().await;
    
        if let Some(client) = client_guard.as_mut() {
            match client.get(command).await {
                Ok(Some(response)) => {
                    let response_str = String::from_utf8(response.to_vec())
                        .map_err(|e| format!("Invalid UTF-8 sequence: {}", e))?;
                    println!("response_str {}", response_str);
    
                    // Split the string at the semicolon
                    let parts: Vec<&str> = response_str.split(';').collect();
                    if parts.len() < 1 {
                        return Err("Response format is incorrect.".to_string());
                    }
    
                    // Extract the IP address and coordinates
                    let ip_and_coords: Vec<&str> = parts[0].split(':').collect();
                    if ip_and_coords.len() != 2 {
                        return Err("IP and coordinates format is incorrect.".to_string());
                    }
    
                    let ip = ip_and_coords[0].to_string();
    
                    // Extracting coordinates
                    let coords: Vec<&str> = ip_and_coords[1].split(',').collect();
                    if coords.len() != 2 {
                        return Err("Coordinates format is incorrect.".to_string());
                    }
                    println!("x {}", coords[0]);
                    let x = coords[0]
                    .trim()
                    .parse::<f32>()
                    .map_err(|e| format!("Invalid X coordinate: {}", e))?
                    .round() as i32; // Round and convert to i32
                
                let y = coords[1]
                    .trim()
                    .parse::<f32>()
                    .map_err(|e| format!("Invalid Y coordinate: {}", e))?
                    .round() as i32; // Round and convert to i32
                    Ok((ip, x, y))
                }
                Ok(None) => Err("No data received for player position.".to_string()),
                Err(e) => Err(format!("Error retrieving player position: {}", e)),
            }
        } else {
            Err("Client not connected, unable to get player update.".to_string())
        }
    }
}
