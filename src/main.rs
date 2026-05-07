// mod models;
mod handlers;
mod routes;
mod schema;
mod middleware;
mod utils;
use schema::{AppState};
use std::time::Duration;

use dashmap::DashMap;
use std::sync::Arc;

use routes::user_routes;
use tower_http::cors::CorsLayer;

use axum::{
    routing::{get, post},
    Router,
    http::{StatusCode, Method},
};
use crate::handlers::{create_user_handler, auth_user_handler };


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
    let active_bots = Arc::new(DashMap::new());

    
    let origins = [
        "http://localhost:4200".parse().unwrap(),
    ];

    let layer = CorsLayer::new().allow_origin(origins).allow_methods([Method::GET, Method::POST]).allow_headers([axum::http::header::CONTENT_TYPE]);

    // Connexion witch postgress
    let mut retry_count = 0;
    let max_retries = 10;

    let pool = loop {
        match PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(30))
            .connect(&db_url)
            .await 
        {
            Ok(p) => {
                println!("✅ Connexion à la base de données réussie !");
                break p;
            },
            Err(e) => {
                retry_count += 1;
                println!("❌ ÉCHEC CONNEXION : {:?}", e);
                println!("⚠️ BDD non prête (tentative {}/{})... On attend 2s", retry_count, max_retries);
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    };
    // Lancement des migrations
    tracing::info!("Vérification des migrations...");
    shared::run_migrations(&pool).await.expect("Échec des migrations");

    let state = AppState {
        pool,
        jwt_secret: jwt_secret,
        master_encryption_key,
        active_bots,
    };

    // route publique 
    let public_routes = Router::new()
        .route("/api/user/auth", post(auth_user_handler))
        .route("/api/user/signup", post(create_user_handler));

    // routes protégées
    let protected_routes = Router::new()
        .route("/health", get(|| async { StatusCode::OK }))
        .nest("/api/user", user_routes())
        .nest("/api/bot", bot_routes())
        .route_layer(axum::middleware::from_fn_with_state(state.clone(), crate::middleware::auth::auth_middleware));

    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)  
        .layer(layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}