use actix_web::{
    Responder, get, post,
    web::Json, HttpResponse, 
    http::header::ContentType
};
use crate::app::response::Response;
use crate::model::repository::champion::{Champion, ChampionOut};

#[get("/champion")]
pub async fn get_champion() -> impl Responder {
    let champ = Champion {
        name: "Genji Shimada".to_string(),
        role: "Damage".to_string(),
        health: 200,
        affiliation: "Overwatch".to_string(),
    };
    let response = serde_json::to_string(&champ).unwrap();
    HttpResponse::Ok()
       .content_type(ContentType::json())
       .body(response)
}

#[post("/champion")]
pub async fn create_champion(champion: Json<Champion>) -> impl Responder {
    let res = Response {
        msg: "Champion created successfully".to_string(),
        data: Some(ChampionOut {name: champion.name.clone()})
    };
    let res = serde_json::to_string(&res).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(res)
}