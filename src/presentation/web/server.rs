use tokio::net::{ TcpListener };
use axum::{ Router };

pub async fn run(app: Router) {
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}