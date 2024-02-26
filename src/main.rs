use axum::{routing::get, Router};
use axum::http::header;
use axum::response::IntoResponse;
use headless_chrome::{Browser, FetcherOptions, LaunchOptionsBuilder};
use tracing::info;

async fn hello_world() -> impl IntoResponse {
    let browser = Browser::new(LaunchOptionsBuilder::default().fetcher_options(FetcherOptions::default()).build().unwrap()).unwrap();
    let tab = browser.new_tab().unwrap();

    ()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}
