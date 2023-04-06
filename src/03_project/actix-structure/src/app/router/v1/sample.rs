use actix_web::{
    Responder, get, post,
    web::{Data, Json}, 
    HttpResponse
};
use crate::app::response::Response;
use model::repository::champion::{Champion, ChampionOut};

#[get("/champion")]
pub async fn get_champion() -> impl Responder {
    Champion {
        name: "Genji Shimada".to_string(),
        role: "Damage".to_string(),
        health: 200,
        affiliation: "Overwatch".to_string(),
    }
}

#[post("/champion")]
pub async fn create_champion(champion: Json<Champion>) -> impl Responder {
    Response {
        msg: "Champion created successfully".to_string(),
        data: Some(
            ChampionOut {
                name: champion.name.clone(),
            }
        )
    }
}