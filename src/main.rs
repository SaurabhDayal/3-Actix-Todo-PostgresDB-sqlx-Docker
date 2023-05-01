use actix_web::web;
use actix_web::{delete, get, post, put, Responder};
use actix_web::{web::Data, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{self};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
struct Todo {
    user_id: i32,
    user_name: String,
    description: String,
    date: String,
    time: String,
}

pub struct AppState {
    db: Pool<Postgres>,
}

#[post("/todo")]
async fn create(state: Data<AppState>, todo: web::Json<Todo>) -> impl Responder {
    match sqlx::query_as::<_, Todo>(
        // match sqlx::query(
        "INSERT INTO todos (user_name, description, date, time) VALUES ($1, $2, $3, $4) RETURNING user_id, user_name, description, date, time"
    )
        .bind(todo.user_name.to_string())
        .bind(todo.description.to_string())
        .bind(todo.date.to_string())
        .bind(todo.time.to_string())
        .fetch_one(&state.db)
        .await
    {
        Ok(todo) => {
            // let todo: Todo = todo.
            HttpResponse::Ok().json(todo)},
        Err(y) => {
            println!("{:?}",y);
            HttpResponse::InternalServerError().json("Failed to create user article")}
    }
}

#[get("/todo")]
async fn get_todos(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, Todo>("SELECT user_id, user_name, description, date, time FROM todos")
        .fetch_all(&state.db)
        .await
    {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::NotFound().json("No Todos found"),
    }
}

#[get("/todo/{id}")]
async fn get_todo_by_id(state: Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let id = id.into_inner();

    match sqlx::query_as::<_, Todo>(
        "SELECT user_id, user_name, description, date, time FROM todos WHERE user_id=$1",
    )
    .bind(id)
    .fetch_one(&state.db)
    .await
    {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().json("Failed to get user todo"),
    }
}

#[put("todo/{id}")]
async fn modify_by_id(
    state: Data<AppState>,
    id: web::Path<i32>,
    todo: web::Json<Todo>,
) -> impl Responder {
    let id = id.into_inner();
    let todo = todo.into_inner();
    match sqlx::query_as::<_, Todo>(
        "UPDATE todos SET user_name=$1, description=$2, date=$3, time=$4 WHERE
         user_id=$5 RETURNING user_id, user_name, description, date, time"
    )
        .bind(&todo.user_name)
        .bind(&todo.description)
        .bind(&todo.date)
        .bind(&todo.time)
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().json("Failed to update user todo"),
    }
}

#[delete("todo/{id}")]
async fn delete_by_id(state: Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let id = id.into_inner();
    match sqlx::query_as::<_, Todo>(
        "DELETE FROM todos WHERE user_id=$1 RETURNING user_id, user_name, description, date, time",
    )
    .bind(id)
    .fetch_one(&state.db)
    .await
    {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create user article"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = Data::new(
        AppState {  
            db:
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Error building a connection pool")}
    );

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(create)
            .service(get_todos)
            .service(get_todo_by_id)
            .service(modify_by_id)
            .service(delete_by_id)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
