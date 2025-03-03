use std::sync::{Arc};

use axum::{extract::{ws::{Message, WebSocket}, State, WebSocketUpgrade}, response::IntoResponse, routing::{get, post}, Extension, Json, Router};
use futures_util::{SinkExt, StreamExt};
use hyper::Method;
use tokio::sync::{broadcast::{self, Sender}, Mutex};
use tower_http::cors::{Any, CorsLayer};

use crate::core::{cartas::{compra_cartas, curar_personagem, dano_bang, descartar_carta}, iniciar_jogo, jogos::{self, carrega_jogos, carregar_jogo, entrar_jogo, passar_turno}, personagens::lista_personagens, AppState, Jogo};
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

async fn atualizar_estado(Extension(state): Extension<Arc<AppState>>, ws: WebSocketUpgrade, State(wsstate): State<WebSocketState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_atualizar(Extension(state.clone()), socket, wsstate))
}

#[derive(Debug, Clone)]
pub struct WebSocketState {
    pub broadcast_tx: Arc<Mutex<Sender<Message>>>,
}

pub async fn handle_atualizar(
    Extension(state): Extension<Arc<AppState>>, 
    socket: WebSocket, 
    wsstate: WebSocketState
) {
    let (ws_tx, mut ws_rx) = socket.split();
    let ws_tx = Arc::new(Mutex::new(ws_tx));

    let broadcast_rx = wsstate.broadcast_tx.lock().await.subscribe();
    tokio::spawn(async move {
        while let Some(result) = ws_rx.next().await {
            match result {
                Ok(msg) => {
                    match msg.into_text() {
                        Ok(id_str) => {
                            if id_str == "keep-alive"{
                                println!("keep-alive recebido no handle_atualizar");
                                break;
                            }
                            println!("id_str: {}", id_str);
                            let jogos_list = carrega_jogos(&state).await;
                            let id: u32 = id_str.parse().unwrap();
                            let jogo = jogos_list.iter().find(|jogo| jogo.id == id).unwrap();
                            if ws_tx.lock().await.send(Message::Text(jogo.id.to_string().into())).await.is_err() {
                                eprintln!("Erro ao enviar mensagem no handle_atualizar.");
                                    break;
                            }
                            println!("Mensagem enviada pelo handle_atualizar");
                        }
                        Err(err) => {
                            eprintln!("Erro ao converter mensagem para texto: {:?}", err);
                            break;
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Erro ao receber mensagem WebSocket: {:?}", err);
                    break;
                }
            }
        }
        println!("Fim do handle_atualizar.");
    });
}



pub async fn handle_atualizar2(Extension(state): Extension<Arc<AppState>>,  socket: WebSocket, wsstate: WebSocketState) {
    
    let (ws_tx, mut ws_rx) = socket.split();
    let ws_tx = Arc::new(Mutex::new(ws_tx));

    
    {
        let broadcast_rx = wsstate.broadcast_tx.lock().await.subscribe();
        tokio::spawn(async move {
            
            while let Some(msg) = ws_rx.next().await {
                println!("ID recebido: {:?}", msg);
        
                if let Ok(msg) = msg {
                    println!("ID decodificado com sucesso.");
        
                    let jogos_list = carrega_jogos(&state).await;
        
                    let idmsg = msg.into_text().unwrap().to_string();
                    let id: u32 = idmsg.parse().unwrap();
        
                    let jogo = jogos_list.iter().find(|jogo| jogo.id == id).unwrap();
        
                    if ws_tx.lock().await.send(Message::Text(jogo.id.to_string().into())).await.is_err() {  
                        println!("Erro ao enviar mensagem no handle_atualizar.");
                    }            
                } else {
                    println!("Erro ao decodificar mensagem no handle_atualizar.");
                }
                break;
            }    
        
            println!("Fim do handle_atualizar.");
            
        });
    }

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
    let (tx, _) = broadcast::channel(32);
    let wsstate = WebSocketState {
        broadcast_tx: Arc::new(Mutex::new(tx)),
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
        .route("/passar_turno", post(passar_turno))
        .route("/descartar_carta", post(descartar_carta))
        .route("/atualizar_estado", get(atualizar_estado))
        .route("/curar_personagem", post(curar_personagem))
        .route("/dano_bang", post(dano_bang))

        .layer(Extension(Arc::new(app_state)))
        .with_state(wsstate)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(vec![Method::POST, Method::PUT, Method::OPTIONS, Method::GET]) 
                .allow_headers(Any)
        );
    app
}
