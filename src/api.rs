use axum::Json;
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};

#[derive(Serialize, ToSchema)]
pub struct Info {
    pub service: &'static str,
}

#[utoipa::path(get, path = "/", responses((status = 200, body = Info)))]
pub async fn index() -> Json<Info> {
    Json(Info {
        service: "srvcs-service",
    })
}

#[derive(OpenApi)]
#[openapi(paths(index), components(schemas(Info)))]
pub struct ApiDoc;

/// Serve OpenAPI document
pub async fn openapi_json() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn openapi_documents_index() {
        assert!(ApiDoc::openapi().paths.paths.contains_key("/"));
    }

    #[tokio::test]
    async fn index_returns_service_info() {
        let Json(info) = index().await;
        assert_eq!(info.service, "srvcs-service");
    }
}
