use std::sync::Mutex;

use actix_web::{
    middleware::{DefaultHeaders, Logger},
    web::Data,
    App, HttpServer,
};
use api::{ascents_user, ascents_user_last, ascents_user_reload};
use envconfig::Envconfig;

mod api;
mod datacollector;
mod store;

#[derive(Envconfig)]
struct Config {
    #[envconfig(from = "DEBUG", default = "false")]
    debug: bool,
    #[envconfig(from = "LOG_LEVEL", default = "info")]
    log_level: String,

    #[envconfig(from = "DB_HOST", default = "127.0.0.1")]
    db_host: String,
    #[envconfig(from = "DB_PORT", default = "27017")]
    db_port: u16,
    #[envconfig(from = "DB_USER", default = "root")]
    db_user: String,
    #[envconfig(from = "DB_PASSWORD")]
    db_password: String,

    #[envconfig(from = "HTTP_HOST", default = "localhost")]
    http_host: String,
    #[envconfig(from = "HTTP_PORT", default = "8080")]
    http_port: u16,

    #[envconfig(from = "SIDEXPORTER_USERNAME")]
    sidexporter_username: String,
    #[envconfig(from = "SIDEXPORTER_PASSWORD")]
    sidexporter_password: String,
}

#[actix_web::main]
async fn main() {
    let config = Config::init_from_env().unwrap();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or(config.log_level));

    // TODO: pass authenticator path via env variable
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
            .wrap(DefaultHeaders::new().add(("Access-Control-Allow-Origin", "http://localhost:5173"))) // TODO: Once I have nginx in front, I shouldn't need this anymore
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
