#[cfg(test)]
mod tests {
    use actix_web::{test, App, web::{self, Data}};
    use crate::app::{router, response::Response};
    use crate::model::repository::champion::Champion;
    use crate::config::AppConfig;

    const PATH: &str = "/api/v1";

    #[actix_web::test]
    async fn test_true() {
        assert!(true);
    }

    #[actix_web::test]
    async fn test_index() {
        let app_config = AppConfig::from_env(".env");
        let app = test::init_service(
            App::new()
            .app_data(Data::new(app_config.clone()))
            .service(router::template::index)
        ).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_champion() {
        let app = test::init_service(
            App::new()
            .service(web::scope(PATH)
                .configure(router::v1::init))
        ).await;
        let req = test::TestRequest::get()
            .uri(&format!("{}/champion", PATH)).to_request();
        let resp: Response<Champion> = test::call_and_read_body_json(&app, req).await;
        assert_eq!(
            resp.data.unwrap().name, 
            "Genji Shimada".to_string()
        );
    }
}