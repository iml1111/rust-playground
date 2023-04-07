use actix_web::{
    Responder, get, post, put,
    web::{Json, Path},
};
use crate::app::response::json_response;
use crate::controller::calc;
use crate::model::repository::champion::{Champion, ChampionOut};

#[get("/champion")]
pub async fn get_champion() -> impl Responder {
    let champ = Champion {
        name: "Genji Shimada".to_string(),
        role: "Damage".to_string(),
        health: 200,
        affiliation: "Overwatch".to_string(),
    };
    json_response::ok(champ)
}

#[post("/champion")]
pub async fn create_champion(champion: Json<Champion>) -> impl Responder {
    let res = ChampionOut {name: champion.name.clone()};
    json_response::created(res)
}

#[put("/calc/add/{num1}/{num2}")]
pub async fn calc_add(nums: Path<(i32, i32)>) -> impl Responder {
    let (num1, num2) = nums.into_inner();
    json_response::ok(serde_json::json!({
        "data": calc::add(num1, num2)
    }))
}

#[get("/error/bad_request")]
pub async fn bad_request() -> impl Responder {
    json_response::bad_request(Some("Something Bad!".to_string()))
}

#[get("/error/forbidden")]
pub async fn forbidden() -> impl Responder {
    json_response::forbidden(Some("Something Forbidden!".to_string()))
}