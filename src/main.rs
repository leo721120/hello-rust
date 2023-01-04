mod app;
mod foo;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let ctx = app::App {
        pool: sqlx::any::AnyPoolOptions::new()
            .max_connections(5)
            .connect("sqlite::memory:")
            .await
            .expect("fail to create db pool"),
    };

    let app = axum::Router::new()
        .route("/foo", axum::routing::post(foo::controller::create_foo))
        .with_state(ctx);

    let srv = axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(app.into_make_service());

    if let Err(err) = srv.await {
        tracing::warn!("{}", err);
    }
}
