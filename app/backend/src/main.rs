
use std::sync::Mutex;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::{ascents_user_last, ascents_user_reload};

mod api;
mod store;

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // TODO: Will have to think about this client here how it should work with the actix framework
    //          Even though it's limiting, I'd rather just have a single instance of it
    //          to not get blocked out spamming requests.
    let client = data_collector::new_client(true, "../../sidexporter/main.mjs".to_string());
    let mt = Mutex::new(client);
    let client_extractor = Data::new(mt);
    // let user_ascents = client.user_ascents("antoni-mleczko", &data_collector::ClimbingCategory::SportClimbing, false).unwrap();
    // client.authenticate().unwrap();
    // let user_ascents = client.user_ascents("antoni-mleczko", &data_collector::ClimbingCategory::SportClimbing, false).unwrap();
    // println!("{:?}", user_ascents);

    let db = store::Mongo::new("mongodb://root:example@127.0.0.1:27017")
        .await
        .unwrap();
    let db_extractor = Data::new(db);

    HttpServer::new(move || {
        App::new()
            // TODO: I'm too noob at rust to know how to not make these unneccesary clones
            .app_data(client_extractor.clone())
            .app_data(db_extractor.clone()) 
            .wrap(Logger::default())
            .service(ascents_user_last)
            .service(ascents_user_reload)
    })
    .bind(("localhost", 8080))
    .unwrap()
    .run()
    .await
    .unwrap();
}

fn example_ascents_static() -> Vec<serde_json::Value> {
    let asc1_str = r#"
    {
        "ascentId": 5590446,
        "platform": "eight_a",
        "userAvatar": "https://d3byf4kaqtov0k.cloudfront.net/p/gallery/thumb/dokdghsf.webp",
        "userName": "Antoni Mleczko",
        "userSlug": "antoni-mleczko",
        "date": "2020-05-03T00:00:00+00:00",
        "difficulty": "6a+",
        "comment": null,
        "userPrivate": false,
        "countrySlug": "poland",
        "countryName": "Poland",
        "areaSlug": "jura-krakowsko-czestochowska",
        "areaName": "Jura Krakowsko - Częstochowska",
        "sectorSlug": "lyse-skaly",
        "sectorName": "Łyse Skały",
        "traditional": false,
        "firstAscent": false,
        "chipped": false,
        "withKneepad": false,
        "badAnchor": false,
        "badBolts": false,
        "highFirstBolt": false,
        "looseRock": false,
        "badClippingPosition": false,
        "isHard": false,
        "isSoft": false,
        "isBoltedByMe": false,
        "isOverhang": false,
        "isVertical": false,
        "isSlab": false,
        "isRoof": false,
        "isAthletic": false,
        "isEndurance": false,
        "isCrimpy": false,
        "isCruxy": false,
        "isSloper": false,
        "isTechnical": false,
        "type": "rp",
        "repeat": false,
        "project": false,
        "rating": 0,
        "category": 0,
        "recommended": false,
        "secondGo": true,
        "duplicate": false,
        "isDanger": false,
        "zlagGradeIndex": 17,
        "zlaggableName": "Piruet",
        "zlaggableSlug": "piruet-71d30",
        "cragSlug": "dolina-szklarki",
        "cragName": "Dolina Szklarki"
    }
    "#;

    let asc2_str = r#"
    {
        "ascentId": 5826600,
        "platform": "eight_a",
        "userAvatar": "https://d3byf4kaqtov0k.cloudfront.net/p/gallery/thumb/dokdghsf.webp",
        "userName": "Antoni Mleczko",
        "userSlug": "antoni-mleczko",
        "date": "2020-08-30T00:00:00+00:00",
        "difficulty": "5a",
        "comment": "",
        "userPrivate": false,
        "countrySlug": "poland",
        "countryName": "Poland",
        "areaSlug": "jura-krakowsko-czestochowska",
        "areaName": "Jura Krakowsko - Częstochowska",
        "sectorSlug": "skala-zachwytu",
        "sectorName": "Skała Zachwytu",
        "traditional": false,
        "firstAscent": false,
        "chipped": false,
        "withKneepad": false,
        "badAnchor": false,
        "badBolts": false,
        "highFirstBolt": false,
        "looseRock": false,
        "badClippingPosition": false,
        "isHard": false,
        "isSoft": false,
        "isBoltedByMe": false,
        "isOverhang": false,
        "isVertical": true,
        "isSlab": true,
        "isRoof": false,
        "isAthletic": false,
        "isEndurance": false,
        "isCrimpy": false,
        "isCruxy": false,
        "isSloper": false,
        "isTechnical": false,
        "type": "os",
        "repeat": false,
        "project": false,
        "rating": 0,
        "category": 0,
        "recommended": false,
        "secondGo": false,
        "duplicate": false,
        "isDanger": false,
        "zlagGradeIndex": 10,
        "zlaggableName": "Pejzaż Horyzontalny",
        "zlaggableSlug": "pejzaz-horyzontalny",
        "cragSlug": "dziurawa-95lxd",
        "cragName": "Dolina Prądnika"
    }
    "#;

    let asc3_str = r#"
    {
        "ascentId": 5832108,
        "platform": "eight_a",
        "userAvatar": "https://d3byf4kaqtov0k.cloudfront.net/p/gallery/thumb/dokdghsf.webp",
        "userName": "Antoni Mleczko",
        "userSlug": "antoni-mleczko",
        "date": "2020-09-02T00:00:00+00:00",
        "difficulty": "6a",
        "comment": "",
        "userPrivate": false,
        "countrySlug": "poland",
        "countryName": "Poland",
        "areaSlug": "jura-krakowsko-czestochowska",
        "areaName": "Jura Krakowsko - Częstochowska",
        "sectorSlug": "witkowe-ptasia-turnia-34e5l",
        "sectorName": "Witkowe - Ptasia Turnia",
        "traditional": false,
        "firstAscent": false,
        "chipped": false,
        "withKneepad": false,
        "badAnchor": false,
        "badBolts": false,
        "highFirstBolt": false,
        "looseRock": false,
        "badClippingPosition": false,
        "isHard": false,
        "isSoft": false,
        "isBoltedByMe": false,
        "isOverhang": true,
        "isVertical": true,
        "isSlab": true,
        "isRoof": false,
        "isAthletic": true,
        "isEndurance": false,
        "isCrimpy": false,
        "isCruxy": false,
        "isSloper": false,
        "isTechnical": false,
        "type": "os",
        "repeat": false,
        "project": false,
        "rating": 5,
        "category": 0,
        "recommended": false,
        "secondGo": false,
        "duplicate": false,
        "isDanger": false,
        "zlagGradeIndex": 16,
        "zlaggableName": "Ornitopresja",
        "zlaggableSlug": "ornitopresja",
        "cragSlug": "dolina-szklarki",
        "cragName": "Dolina Szklarki"
    }
    "#;

    vec![
        serde_json::from_str(asc1_str).unwrap(),
        serde_json::from_str(asc2_str).unwrap(),
        serde_json::from_str(asc3_str).unwrap(),
    ]
}
