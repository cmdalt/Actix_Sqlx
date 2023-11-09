use dotenv::dotenv;
use std::env;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder,web::Data};
use sqlx::{postgres::PgPoolOptions,Pool,Postgres,self,FromRow};

pub struct AppState {
    db: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(hello)
            .service(fetch_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, FromRow)]
struct User {
    actor_id: i32,
    first_name: String,
    last_name: String,
}

#[get("/users")]
pub async fn fetch_users(state: Data<AppState>) -> impl Responder {
    // "GET /users".to_string()

    match sqlx::query_as::<_, User>("SELECT actor_id, first_name, last_name FROM actor")
        .fetch_all(&state.db)
        .await
    {
        Ok(userst) => HttpResponse::Ok().json(userst),
        Err(_) => HttpResponse::NotFound().json("No users found"),
    }
}
