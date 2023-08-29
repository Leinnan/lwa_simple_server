use clap::{command, Parser};
use std::path::PathBuf;

/// Simple server made with hosting locally webgl games in mind.
#[derive(Debug, Parser, Clone)]
#[command(name = "lwa_simple_server", version = "0.1.0", author = "Mev Lyshkin")]
pub struct SimpleServer {
    /// Folder to host, current by default
    pub folder_to_host: Option<PathBuf>,
    /// Should use SSL, false by default
    #[arg(long)]
    pub ssl: bool,
    /// Specifies hosting address, "localhost:8080" by default
    #[arg(short, long)]
    pub address: Option<String>,
    // Specifies folder containing "key.pem" and "cert.pem" required for ssl hosting, defaults to current folder
    #[arg(short, long)]
    certificates_folder: Option<PathBuf>,
}

impl SimpleServer {
    pub fn get_folder_to_host(&self) -> PathBuf {
        if self.folder_to_host.is_some() {
            self.folder_to_host.clone().unwrap()
        } else {
            PathBuf::from(".")
        }
    }
    pub fn get_address(&self) -> String {
        if self.address.is_some() {
            return self.address.clone().unwrap();
        }
        "localhost:8080".to_string()
    }

    fn get_certificates_folder(&self) -> PathBuf {
        if self.certificates_folder.is_some() {
            self.certificates_folder.clone().unwrap()
        } else {
            PathBuf::from(".")
        }
    }

    pub fn get_private_key_path(&self) -> PathBuf {
        std::fs::canonicalize(self.get_certificates_folder().join("key.pem")).unwrap()
    }

    pub fn get_certificate_chain_file_path(&self) -> PathBuf {
        std::fs::canonicalize(self.get_certificates_folder().join("cert.pem")).unwrap()
    }
}
