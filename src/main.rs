#[macro_use]
extern crate serde_derive;
extern crate confy;
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{middleware, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::env;

mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    let cfg: config::ServerConfig = confy::load("lwa_simple_server").unwrap();
    {
        println!("The configuration is:\n{:#?}", cfg);
        // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`

        println!(
            "Starting server on address: {} with hosted folder: {} wit SSL: {}",
            cfg.bind_address, cfg.folder_to_host, cfg.use_ssl
        );
    }

    let server = HttpServer::new(move || {
        let cfg: config::ServerConfig = confy::load("lwa_simple_server").unwrap();
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::default())
            .service(fs::Files::new("/", &cfg.folder_to_host).index_file("index.html"))
    });

    if cfg.use_ssl {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file(&cfg.private_key_file, SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file(&cfg.certificate_chain_file).unwrap();

        server.bind_openssl(&cfg.bind_address, builder)?
        .run()
        .await
    } else {
        server.bind(&cfg.bind_address)?
        .run()
        .await
    }
}
