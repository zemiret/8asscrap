use std::sync::Mutex;

use actix_web::{get, post, web::{Data, Path}, HttpResponse, Responder};
use data_collector::{Client, ClimbingCategory};

use crate::store::Mongo;

const LAST_ASCENTS_COUNT: u8 = 3;

#[get("/api/v1/ascents/{user}/last")]
async fn ascents_user_last(db: Data<Mongo>, path: Path<(String,)>) -> impl Responder {
    let user = &path.0;
    match db.user_peek_ascents(user, u32::from(LAST_ASCENTS_COUNT)).await {
        Ok(ascents) => {
            HttpResponse::Ok().json(ascents)
        },
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("db.user_peek_ascents({}): {}", user, err))
        }
    }
}

#[post("/api/v1/ascents/{user}/reload")]
async fn ascents_user_reload(path: Path<(String,)>, db: Data<Mongo>, client: Data<Mutex<Client>>) -> impl Responder {
    let user = &path.0;

    // TODO: Handle error nicer than unwrap
    // TODO: For now only sport climbing
    // TODO: Handle reauthentication - have to have env passed in from main

    // client here is shared across all workers. Might become a big bottleneck. 
    // But - I also do not want more instances, bc I'm afraid of bot protection.
    let mut client = client.lock().unwrap();
    let ascents = client.user_ascents(user, &ClimbingCategory::SportClimbing, false).unwrap();
    db.user_replace_ascents(user, ascents).await.unwrap();

    HttpResponse::Ok()
}