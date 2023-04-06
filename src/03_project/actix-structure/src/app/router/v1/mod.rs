use actix_web::web::{self, service};
pub mod sample;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(sample::get_champion)
        .service(sample::create_champion)
        .service(sample::calc_add);
}