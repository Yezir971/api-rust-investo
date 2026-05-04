mod models;
mod handlers;
mod routes;
mod schema;

use routes::user_routes;

use axum::{
    routing::get,
    Router,
    http::StatusCode,
};
use sqlx::postgres::PgPoolOptions;




#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DB_URL manquante");

    // Connexion avec postgress
    let pool = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Échec connexion BDD");


    // build our application with a single route
    let app = Router::new()
        .route("/health", get(|| async {StatusCode::OK}))
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/api/user",user_routes() )
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}