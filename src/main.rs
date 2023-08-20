use std::{net::SocketAddr, path::Path, str::FromStr, time::Duration};

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use notify::RecursiveMode;
use notify_debouncer_mini::{new_debouncer, DebounceEventResult};
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

async fn fallback() -> impl IntoResponse {
    Html("<h3>Oops! The page probably refreshed before the build finished.</h3>")
}

async fn hello() -> impl IntoResponse {
    "<h1>Hello!</h1>"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();

    let web_routes = Router::new().route("/hello", get(hello));

    let app = Router::new()
        .nest("/web", web_routes)
        .nest_service("/", ServeDir::new("./web/dist").fallback(get(fallback)))
        .layer(livereload);

    let mut debouncer = new_debouncer(
        Duration::from_millis(100),
        move |res: DebounceEventResult| match res {
            Ok(_) => reloader.reload(),
            Err(e) => tracing::error!("Watcher (debouncer) Error {:?}", e),
        },
    )
    .unwrap();

    debouncer
        .watcher()
        .watch(Path::new("./web/dist"), RecursiveMode::Recursive)
        .unwrap();

    axum::Server::bind(&SocketAddr::from_str("127.0.0.1:3000").unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
