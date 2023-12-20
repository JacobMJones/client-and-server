// network_client.rs
use mini_redis::client::Client;
use mini_redis::client;
use local_ip_address::local_ip;
use bytes::Bytes;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct NetworkClient {
    server_address: String,
    client: Arc<Mutex<Option<Client>>>, 
}

impl NetworkClient {
    pub fn initialize(server_address: &str) -> Self {
        NetworkClient { 
            server_address: server_address.to_string(),
            client: Arc::new(Mutex::new(None))
        }
    }
    pub async fn connect(&self) {
        let mut locked_client = self.client.lock().await;
        *locked_client = Some(client::connect(&self.server_address).await.unwrap());
    }
    pub async fn set_player_position(&mut self, position: &str) {
        let formatted_ip = format!("{}", local_ip().unwrap());
        let position_bytes = Bytes::from(position.to_owned()); // Clone the data into a new Bytes object

        let mut client_guard = self.client.lock().await;
        if let Some(client) = client_guard.as_mut() {
            client.set(&formatted_ip, position_bytes).await.unwrap();
        } else {
            // Handle the case where client is None
            // e.g., log an error or return a Result indicating the error
        }
    }

    pub async fn get_player_update(&mut self) -> (i32, i32) {
        let mut client_guard = self.client.lock().await;
        if let Some(client) = client_guard.as_mut() {
            if let Ok(Some(player_pos_raw)) = client.get("PUD").await {
                if let Ok(player_pos_str) = String::from_utf8(player_pos_raw.to_vec()) {
                    let parts: Vec<&str> = player_pos_str.split(':').collect();
                    if parts.len() == 2 {
                        let coordinates: Vec<&str> = parts[1].split(',').collect();
                        if coordinates.len() == 2 {
                            if let (Ok(x), Ok(y)) = (
                                coordinates[0].trim().parse::<i32>(),
                                coordinates[1].trim().parse::<i32>()
                            ) {
                                return (x, y);
                            }
                        }
                    }
                }
            }
        }
        panic!("Invalid player position data or client not connected")
    }
}
