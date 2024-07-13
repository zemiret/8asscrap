use actix_web::{get, post, web::{Data, Path}, HttpResponse, Responder};

use crate::store::Mongo;

const LAST_ASCENTS_COUNT: u8 = 3;

#[get("/api/v1/ascents/{user}/last")]
async fn ascents_user_last(db: Data<Mongo>, path: Path<(String,)>) -> impl Responder {
    let user = &path.0;
    if user.is_empty() {
        return HttpResponse::BadRequest().body("empty user parameter");
    }

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
async fn ascents_user_reload(db: Data<Mongo>) -> impl Responder {
    // TODO: Here
    HttpResponse::Ok().body("oko")
}