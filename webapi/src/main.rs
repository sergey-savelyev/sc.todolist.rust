use std::sync::Arc;

use axum::{
    routing::{get, post, patch, delete},
    Router,
    http::{
        header,
        HeaderValue
    }
};
use dotenv::dotenv;
use tower_http::cors::{CorsLayer, Any};

use infrastructure::ServiceProvider;

pub mod tasks_handle;
pub mod logs_handle;
pub mod view;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5454".parse::<HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers([header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE]);

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let app = 
        Router::new()
            .route("/api/tasks", get(tasks_handle::get_tasks_batch))
            .route("/api/tasks", post(tasks_handle::create_task))
            .route("/api/tasks/:id", get(tasks_handle::get_task))
            .route("/api/tasks/:id", patch(tasks_handle::update_task))
            .route("/api/tasks/:id", delete(tasks_handle::delete_task))
            .route("/api/tasks/search/:phrase", get(tasks_handle::search_tasks))
            .route("/api/tasks/:id/root", patch(tasks_handle::change_task_root))

            .route("/api/tasks/:id/logs", get(logs_handle::get_task_logs))
            .route("/api/tasks/logs", get(logs_handle::get_all_logs))

            .with_state(Arc::new(ServiceProvider::new(&database_url)))
            .layer(cors);

    axum::Server::bind(&"0.0.0.0:3005".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}