mod domain;
mod infrastructure;
mod application;
mod presentation;
mod bootstrap;

#[tokio::main]
async fn main() {
    let db_pool = bootstrap::create_mysql_pool().await.expect("Database connection failed");
    let provider = bootstrap::create_mysql_provider().await;
    let uow_factory = bootstrap::create_mysql_unit_of_work_factory(db_pool);
    let ucs = bootstrap::build_use_cases(uow_factory, provider);

    let state = presentation::web::AppState::new(ucs);
    let app = presentation::web::create_router(state);

    presentation::web::run_server(app).await;
}
