use crate::client_manager::client_manager::ClientManager;

pub struct Server {
    server_name: String,
    address: String,
}

impl Server {
    pub fn new(server_name: String, address: String) -> Self {
        Self {
            server_name,
            address,
        }
    }

    pub fn start(&self) {
        let listener = std::net::TcpListener::bind(&self.address);
        match listener {
            Ok(listener) => {
                println!("Server {} started at {}", self.server_name, self.address);
                let client_manager = ClientManager::new();
                client_manager.start(listener);
            }
            Err(e) => {
                println!("Failed to start server : {}", e);
            }
        }
    }
}
