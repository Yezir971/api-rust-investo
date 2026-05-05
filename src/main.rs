// mod models;
mod handlers;
mod routes;
mod schema;
use schema::{AppState};

use routes::user_routes;
use tower_http::cors::CorsLayer;

use axum::{
    routing::get,
    Router,
    http::{StatusCode, Method},
};
use sqlx::postgres::PgPoolOptions;

use dotenvy::dotenv;
use std::env;

use crate::routes::bot_routes;



#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL manquant");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET manquant");
    let master_encryption_key = env::var("MASTER_ENCRYPTION_KEY").expect("MASTER_ENCRYPTION_KEY manquant");


    let origins = [
        "http://localhost:4200".parse().unwrap(),
    ];

    let layer = CorsLayer::new().allow_origin(origins).allow_methods([Method::GET, Method::POST]).allow_headers([axum::http::header::CONTENT_TYPE]);

    // Connexion witch postgress
    let pool = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Échec connexion BDD");


    let state = AppState {
        pool,
        jwt_secret: jwt_secret,
        master_encryption_key,
    };

    // build our application with a single route
    let app = Router::new()
        .route("/health", get(|| async {StatusCode::OK}))
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/api/user",user_routes() )
        .nest("/api/bot", bot_routes())
        .with_state(state)
        .layer(layer);



    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}