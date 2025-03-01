use std::sync::{Arc};

use axum::{extract::{ws::{Message, WebSocket}, WebSocketUpgrade}, response::IntoResponse, routing::{get, post}, Extension, Json, Router};
use hyper::Method;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

use crate::core::{cartas::compra_cartas, iniciar_jogo, jogos::{self, carrega_jogos, carregar_jogo, entrar_jogo}, personagens::lista_personagens, AppState, Jogo};
use crate::core::jogos::usa_carta;

async fn printa_jogos(state: &Arc<AppState>){
    let jogos = carrega_jogos(state).await;
    for jogo in jogos{
        println!("ID do jogo no WS: {}", jogo.id);
    }
}

async fn handle_socket(Extension(state): Extension<Arc<AppState>>, mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if let Message::Text(text) = msg {
                println!("{}", text);
                if socket.send(Message::Text(text)).await.is_err() {                    
                    break;
                }
            }
        }
        break;
    }
}

async fn ws_handler(Extension(state): Extension<Arc<AppState>>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(Extension(state.clone()), socket))
}

async fn listar_handler(Extension(state): Extension<Arc<AppState>>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_listar(Extension(state.clone()), socket))
}


pub async fn handle_listar(Extension(state): Extension<Arc<AppState>>, mut socket: WebSocket) {
    printa_jogos(&state).await;
    println!("Socket aberto e pronto para receber mensagens.");

    while let Some(msg) = socket.recv().await {
        println!("Mensagem recebida: {:?}", msg);

        if let Ok(msg) = msg {
            println!("Mensagem decodificada com sucesso.");

            let jogos_list = carrega_jogos(&state).await;
            println!("Jogos carregados");

            let ids: String = jogos_list.iter()
                                        .map(|jogo| jogo.id.to_string())
                                        .collect::<Vec<String>>()
                                        .join("; ");
            println!("IDs gerados: {}", ids);
                                        
            if socket.send(Message::Text(ids.into())).await.is_err() {  
                println!("Erro ao enviar mensagem no handle_listar.");
            }            
        } else {
            println!("Erro ao decodificar mensagem.");
        }
        break;
    }    

    println!("Fim do handle_listar.");
}

pub fn cria_rotas() -> Router<>{
    let app_state = AppState {
        jogos: Arc::new(Mutex::new(Vec::new())),
    };
    let app: Router<_> = Router::new()
        .route("/iniciar_jogo", post(iniciar_jogo))
        .route("/lista_personagens", get(lista_personagens))
        .route("/usa_carta", post(usa_carta))
        .route("/compra_cartas", post(compra_cartas))
        .route("/ws", get(ws_handler))
        .route("/entrar_jogo", post(entrar_jogo))
        .route("/listar_handler", get(listar_handler))
        .route("/carregar_jogo", post(carregar_jogo))

        .layer(Extension(Arc::new(app_state)))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(vec![Method::POST, Method::PUT, Method::OPTIONS, Method::GET]) 
                .allow_headers(Any)
        );
    app
}
