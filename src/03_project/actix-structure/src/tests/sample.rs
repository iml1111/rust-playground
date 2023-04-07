#[cfg(test)]
mod tests {
    use crate::app::router;
    use actix_web::{test, web, App};

    const PATH: &str = "/api/v1";

    #[actix_web::test]
    async fn test_index() {
        let app = test::init_service(
            App::new().service(router::template::index)
        ).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
    
}