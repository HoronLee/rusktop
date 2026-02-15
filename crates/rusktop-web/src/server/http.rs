use std::net::SocketAddr;
use utoipa_swagger_ui::SwaggerUi;
use axum::routing::get;

use crate::service::user_service::UserServiceImpl;

include!(concat!(env!("OUT_DIR"), "/rest_routes.rs"));

pub async fn serve(
    user_service: std::sync::Arc<UserServiceImpl>,
    addr: SocketAddr,
) -> Result<(), std::io::Error> {
    let rest_router = all_rest_routes(user_service);

    let swagger_config = utoipa_swagger_ui::Config::new(["/api-docs/openapi.yaml"]);

    let app = axum::Router::new()
        .merge(rest_router)
        .merge(SwaggerUi::new("/swagger-ui").config(swagger_config))
        .route("/api-docs/openapi.yaml", get(|| async {
            let content = std::fs::read_to_string("api/openapi/v1/openapi.yaml")
                .unwrap_or_else(|_| "openapi: 3.0.0\ninfo:\n  title: Not Found\n  version: 1.0.0\npaths: {}".to_string());
            ([(axum::http::header::CONTENT_TYPE, "application/yaml")], content)
        }));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Web server listening on {}", addr);
    tracing::info!("Swagger UI available at http://{}/swagger-ui", addr);
    axum::serve(listener, app).await
}
