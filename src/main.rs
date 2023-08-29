use actix_cors::Cors;
use actix_files as fs;
use actix_web::{middleware, App, HttpServer};
use clap::Parser;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::env;

mod app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_args = crate::app::SimpleServer::parse();
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    let path = app_args.folder_to_host.clone();
    {
        // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`

        println!(
            "Starting server on address: {}://{} with hosted folder: {} [SSL={}]",
            if app_args.ssl{ "https" } else {"http"},
            app_args.get_address(),
            path,
            app_args.ssl
        );
    }

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(Cors::default())
            .service(
                fs::Files::new("/", &path)
                    .index_file("index.html")
                    .use_last_modified(true),
            )
    });

    if app_args.ssl {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

        if let Err(error) =
            builder.set_private_key_file(&app_args.get_private_key_path(), SslFiletype::PEM)
        {
            eprintln!(
                "Could not read {}: {}",
                app_args.get_private_key_path().display(),
                error
            )
        }
        if let Err(error) =
            builder.set_certificate_chain_file(&app_args.get_certificate_chain_file_path())
        {
            eprintln!(
                "Could not read {}: {}",
                app_args.get_certificate_chain_file_path().display(),
                error
            )
        }

        server
            .bind_openssl(&app_args.get_address(), builder)?
            .run()
            .await
    } else {
        server.bind(&app_args.get_address())?.run().await
    }
}
