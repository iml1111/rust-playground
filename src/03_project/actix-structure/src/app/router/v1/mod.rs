use actix_web::web;
pub mod sample;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(sample::get_champion)
        .service(sample::create_champion);
}