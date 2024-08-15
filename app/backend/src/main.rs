use std::{path::PathBuf, sync::Mutex};

use actix_web::{
    middleware::{DefaultHeaders, Logger},
    web::Data,
    App, HttpServer,
};
use api::{ascents_user, ascents_user_last, ascents_user_reload, ping};
use clap::Parser;
use envconfig::Envconfig;

mod api;
mod datacollector;
mod store;

#[derive(Envconfig)]
#[derive(Debug)]
struct Config {
    #[envconfig(from = "APP_DEBUG", default = "false")]
    debug: bool,
    #[envconfig(from = "APP_LOG_LEVEL", default = "info")]
    log_level: String,

    #[envconfig(from = "APP_DB_HOST", default = "127.0.0.1")]
    db_host: String,
    #[envconfig(from = "APP_DB_PORT", default = "27017")]
    db_port: u16,
    #[envconfig(from = "APP_DB_USER")]
    db_user: String,
    #[envconfig(from = "APP_DB_PASSWORD")]
    db_password: String,

    #[envconfig(from = "APP_HTTP_HOST", default = "localhost")]
    http_host: String,
    #[envconfig(from = "APP_HTTP_PORT", default = "8080")]
    http_port: u16,

    #[envconfig(from = "APP_SIDEXPORTER_USERNAME")]
    sidexporter_username: String,
    #[envconfig(from = "APP_SIDEXPORTER_PASSWORD")]
    sidexporter_password: String,

    #[envconfig(from = "APP_CORS_PATH", default="http://localhost")]
    cors_path: String,
}

#[derive(Parser)]
#[command(about, long_about = None)]
struct Cli {
    /// Filepath for envfile. If passed, program will read the file and load it as env variables.
    #[arg(long, value_name = "FILE")]
    envfile: Option<PathBuf>,
}

#[actix_web::main]
async fn main() {
    let cli = Cli::parse();
    if let Some(filepath) = cli.envfile {
        // Load variables from env file into the environment, but only if its path is passed.
        dotenvy::from_filename(filepath).unwrap();
    }
    let config = Config::init_from_env().unwrap();
    if config.debug {
        println!("Parsed config: {:#?}", config);
    }

    env_logger::init_from_env(env_logger::Env::new().default_filter_or(config.log_level));

    let client = datacollector::Client::new(
        config.debug,
        config.sidexporter_username,
        config.sidexporter_password,
        "sidexporter/main.mjs".to_string(),
    );
    let mt = Mutex::new(client);
    let client_extractor = Data::new(mt);

    let db = store::Mongo::new(
        format!(
            "mongodb://{}:{}@{}:{}",
            config.db_user, config.db_password, config.db_host, config.db_port
        )
        .as_str(),
    )
    .await
    .unwrap();

    let db_extractor = Data::new(db);

    // TODO: low prio. Possibly move factory definition to api module
    HttpServer::new(move || {
        App::new()
            // Self explanation note: So what happens here with the extractors as far as I understand it.
            // The extractors are shared amongst threads wrapped in Arc.
            // As for the db_extractor, we have a single instance, but the mongodb's driver handles connection pool internally,
            // so as we're accessing the collection, the connection pool will be used amongsth threads.
            // As for the client_extractor, I want only 1 instance to be used at a time. So:
            //  1. It is created outside of the closure scope and captured here.
            //      Had it been created here, it would be created every time a handler spawns an App instance
            //     (this closure is a actory function for App instance).
            //  2. It is wrapped in mutex, so only 1 thread is able to access it at a time.
            .app_data(client_extractor.clone())
            .app_data(db_extractor.clone())
            .wrap(Logger::default())
            .wrap(DefaultHeaders::new().add(("Access-Control-Allow-Origin", config.cors_path.as_str()))) // TODO: local development only. Once I have nginx in front, I shouldn't need this anymore
            .service(ping)
            .service(ascents_user)
            .service(ascents_user_last)
            .service(ascents_user_reload)
    })
    .bind((config.http_host, config.http_port))
    .unwrap()
    .run()
    .await
    .unwrap();
}
