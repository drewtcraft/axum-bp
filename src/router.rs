use axum::{response::Html, Router, routing::get};

pub async fn hello_handler() -> Html<&'static str> {
    Html("<p>You's a bitch</p>")
}

pub fn get_routes() -> Router {
    Router::new().route("/yello", get(hello_handler))
}
