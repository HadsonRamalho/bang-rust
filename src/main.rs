pub mod routes;
pub mod core;
#[tokio::main]
async fn main() {
   tracing_subscriber::fmt::init();
   let app = crate::routes::cria_rotas();
   let addr = "0.0.0.0:3069";
   let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
   axum::serve(listener, app).await.unwrap();
}
