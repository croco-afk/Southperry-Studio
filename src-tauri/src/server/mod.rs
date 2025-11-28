pub mod controller;
pub mod extractors;
pub mod middlewares;
pub mod models;

use crate::{store::StringDict, Error};

use axum::{
    http::StatusCode, 
    response::{IntoResponse, Response},
    routing::get,
    serve, Router,
};
use tower_http::cors::{CorsLayer, Any}; 
use wz_reader::WzNodeArc;

pub type AppState = (WzNodeArc, StringDict);

pub async fn app(node: WzNodeArc, string_dict: StringDict, port: u16) -> crate::Result<()> {
    let layer_state = node.clone();
    let app = Router::new()
        .route("/", get(hello))
        .nest("/mapping", controller::mapping_router())
        .nest("/node", controller::node_router())
        .nest("/string", controller::string_router())
        .route_layer(axum::middleware::from_fn_with_state(
            layer_state,
            middlewares::root_check_middleware,
        ))
        .route_layer(axum::middleware::from_fn(
            middlewares::cache_control_from_query_middleware,
        ))
        .route_layer(
            CorsLayer::new()
                .allow_origin(Any)   // 允许任何来源 (解决 8041 端口问题)
                .allow_methods(Any)  // 允许任何方法 (GET, POST, OPTIONS 等)
                .allow_headers(Any), // 允许任何 Header (非常关键！解决 fetch 失败的核心)
        )
        .with_state((node, string_dict));

    let host = format!("127.0.0.1:{port}");

    println!("You enable the axum-server feature, Listening on http://{host}");

    let listener = tokio::net::TcpListener::bind(host).await?;

    serve(listener, app).await.map_err(Error::from)
}

async fn hello() -> &'static str {
    "Hi"
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            // 归类为 500 Internal Server Error 的错误
            Error::Io(_)
            | Error::JsonParseError(_)
            | Error::ImageParseError(_)
            | Error::SoundParseError(_)       // <--- 确保包含
            | Error::StringParseError(_)
            | Error::ImageSendError
            | Error::ImageProcessingError(_)  // <--- 确保包含
            | Error::AudioProcessingError(_)  // <--- 确保包含
            => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
            }

            // 归类为 400 Bad Request 的错误
            Error::InitWzFailed => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            Error::NodeError(_) => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            Error::NodeTypeMismatch(_) => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }

            // 归类为 403 Forbidden 的错误
            Error::NotInitialized => (StatusCode::FORBIDDEN, self.to_string()).into_response(),

            // 归类为 404 Not Found 的错误
            Error::NodeNotFound => (StatusCode::NOT_FOUND, self.to_string()).into_response(),

            // 移动端特有错误
            #[cfg(mobile)]
            Error::PluginInvoke(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response(),
        }
    }
}