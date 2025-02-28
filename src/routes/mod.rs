use axum::{routing::{get, post}, Router};
use hyper::Method;
use tower_http::cors::{Any, CorsLayer};

use crate::core::{iniciar_jogo, personagens::lista_personagens};
use crate::core::jogos::usa_carta;

pub fn cria_rotas() -> Router<>{
    let app: Router<_> = Router::new()
        .route("/iniciar_jogo", post(iniciar_jogo))
        .route("/lista_personagens", get(lista_personagens))
        .route("/usa_carta", post(usa_carta))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(vec![Method::POST, Method::PUT, Method::OPTIONS, Method::GET]) 
                .allow_headers(Any)
        );
    app
}
