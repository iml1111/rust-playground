use actix_web::{
    Responder, get, post, put,
    web::{Json, Path}, HttpResponse, 
    http::header::ContentType
};
use crate::app::response::Response;
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

#[put("/calc/add/{num1}/{num2}")]
pub async fn calc_add(nums: Path<(i32, i32)>) -> impl Responder {
    let (num1, num2) = nums.into_inner();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(
            serde_json::json!({
                "msg": "Calculation successful",
                "data": calc::add(num1, num2)
            }).to_string()
        )
}