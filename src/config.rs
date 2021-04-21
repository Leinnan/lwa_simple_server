#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub folder_to_host: String,
    pub bind_address: String,
    pub private_key_file: String,
    pub certificate_chain_file: String,
    pub use_ssl: bool
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            folder_to_host: String::from(std::env::current_dir().unwrap().to_str().unwrap()),
            bind_address: "localhost:8000".to_string(),
            private_key_file: "key.pem".to_string(),
            certificate_chain_file: "cert.pem".to_string(),
            use_ssl: false,
        }
    }
}
