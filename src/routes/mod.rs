use std::{string::ParseError, sync::Arc};

use axum::{extract::{ws::{Message, WebSocket}, State, WebSocketUpgrade}, response::IntoResponse, routing::{get, post}, Extension, Json, Router};
use futures_util::{SinkExt, StreamExt};
use hyper::Method;
use tokio::sync::{broadcast::{self, Sender}, Mutex};
use tower_http::cors::{Any, CorsLayer};

use crate::core::{cartas::{compra_cartas, curar_personagem, dano_bang, descartar_carta, usar_bang_alvo, DescartaCarta, JogadorCartaAlvo, LogCarta}, iniciar_jogo, jogos::{self, carrega_jogos, carregar_jogo, entrar_jogo, passar_turno}, personagens::lista_personagens, AppState, Jogo};
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

async fn bang_handler(Extension(state): Extension<Arc<AppState>>, ws: WebSocketUpgrade, State(wsstate): State<WebSocketState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_bang(Extension(state.clone()), socket, wsstate))
}

pub async fn handle_bang(
    Extension(state): Extension<Arc<AppState>>, 
    socket: WebSocket, 
    wsstate: WebSocketState
) {
    let (ws_tx, mut ws_rx) = socket.split();
    let ws_tx = Arc::new(Mutex::new(ws_tx));

    let mut broadcast_rx = wsstate.broadcast_tx.lock().await.subscribe();
    
    tokio::spawn(async move {
        loop {
            tokio::select! {
                Some(result) = ws_rx.next() => {
                    match result {
                        Ok(msg) => {
                            match msg.into_text() {
                                Ok(info) => {
                                    if info == "keep-alive-bang" {
                                        println!("keep-alive recebido no handle_bang");
                                        continue; // Mantém o WebSocket aberto
                                    }

                                    println!("info: {}", info);
                                    
                                    // Tenta deserializar a mensagem
                                    let obj: Result<JogadorCartaAlvo, _> = serde_json::from_str(&info);
                                    match obj {
                                        Ok(obj) => {
                                            println!("JogadorCartaAlvo: {}, {}, {}", obj.idjogo, obj.jogador.nome, obj.alvo.nome);
                                            let id = obj.idjogo;

                                            // Chama a função para processar o "bang"
                                            if let Err(err) = usar_bang_alvo(Extension(state.clone()), Json(JogadorCartaAlvo{
                                                jogador: obj.jogador.clone(),
                                                idjogo: obj.idjogo.clone(),
                                                alvo: obj.alvo.clone(),
                                                carta: obj.carta.clone()
                                            })).await {
                                                eprintln!("Erro ao usar bang no alvo: {}", err);
                                                continue;
                                            }

                                            // Carrega a lista de jogos
                                            // enviar info.alvo
                                            let jogos_list = carrega_jogos(&state).await;
                                            if let Some(jogo) = jogos_list.iter().find(|jogo| jogo.id == id) {
                                                let message = obj.into();

                                                // Envia a mensagem para todos os WebSockets via broadcast
                                                if let Err(err) = wsstate.broadcast_tx.lock().await.send(Message::Text(message)) {
                                                    eprintln!("Erro ao enviar mensagem de broadcast: {}", err);
                                                    break;
                                                }
                                                println!("Mensagem enviada para todos os WebSockets.");
                                            } else {
                                                eprintln!("Jogo não encontrado para o id {}", id);
                                            }
                                        }
                                        Err(err) => {
                                            eprintln!("Erro ao converter mensagem para JogadorCartaAlvo: {}", err);
                                            continue;
                                        }
                                    }
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
                Ok(broadcast_msg) = broadcast_rx.recv() => {
                    // Envia mensagens do canal de broadcast para o WebSocket conectado
                    if ws_tx.lock().await.send(broadcast_msg).await.is_err() {
                        eprintln!("Erro ao enviar mensagem de broadcast para o WebSocket");
                        break;
                    }
                }
                else => {
                    // Encerra o loop caso ambas as streams estejam fechadas
                    break;
                }
            }
        }
        println!("Conexão WebSocket encerrada.");
    });
}

async fn uso_carta_handler(Extension(state): Extension<Arc<AppState>>, ws: WebSocketUpgrade, State(wsstate): State<WebSocketState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_uso_carta(Extension(state.clone()), socket, wsstate))
}

pub async fn handle_uso_carta(
    Extension(state): Extension<Arc<AppState>>, 
    socket: WebSocket, 
    wsstate: WebSocketState
) {
    let (ws_tx, mut ws_rx) = socket.split();
    let ws_tx = Arc::new(Mutex::new(ws_tx));

    let mut broadcast_rx = wsstate.broadcast_tx.lock().await.subscribe();
    
    tokio::spawn(async move {
        loop {
            tokio::select! {
                Some(result) = ws_rx.next() => {
                    match result {
                        Ok(msg) => {
                            match msg.into_text() {
                                Ok(info) => {
                                    if info == "keep-alive-uso-carta" {
                                        println!("keep-alive recebido no handle_uso_carta");
                                        continue; // Mantém o WebSocket aberto
                                    }

                                    println!("info: {}", info);
                                    
                                    // Tenta deserializar a mensagem
                                    let obj: Result<DescartaCarta, _> = serde_json::from_str(&info);
                                    match obj {
                                        Ok(obj) => {
                                            println!("handle_uso_carta: {}, {}", obj.idjogo, obj.jogador.nome);
                                            let id = obj.idjogo;                        

                                            // Carrega a lista de jogos
                                            // enviar info.alvo
                                            let jogos_list = carrega_jogos(&state).await;
                                            if let Some(jogo) = jogos_list.iter().find(|jogo| jogo.id == id) {
                                                let message = obj.into();

                                                // Envia a mensagem para todos os WebSockets via broadcast
                                                if let Err(err) = wsstate.broadcast_tx.lock().await.send(Message::Text(message)) {
                                                    eprintln!("Erro ao enviar mensagem de broadcast do handle_uso_carta: {}", err);
                                                    break;
                                                }
                                                println!("Mensagem enviada para todos os WebSockets pelo handle_uso_carta.");
                                            } else {
                                                eprintln!("Jogo não encontrado para o id do handle_uso_carta {}", id);
                                            }
                                        }
                                        Err(err) => {
                                            eprintln!("Erro ao converter mensagem para DescartaCarta: {}", err);
                                            continue;
                                        }
                                    }
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
                Ok(broadcast_msg) = broadcast_rx.recv() => {
                    // Envia mensagens do canal de broadcast para o WebSocket conectado
                    if ws_tx.lock().await.send(broadcast_msg).await.is_err() {
                        eprintln!("Erro ao enviar mensagem de broadcast para o WebSocket");
                        break;
                    }
                }
                else => {
                    // Encerra o loop caso ambas as streams estejam fechadas
                    break;
                }
            }
        }
        println!("Conexão WebSocket encerrada.");
    });
}

#[derive(Debug, Clone)]
pub struct WebSocketState {
    pub broadcast_tx: Arc<Mutex<Sender<Message>>>,
}

async fn toast_handler(Extension(state): Extension<Arc<AppState>>, ws: WebSocketUpgrade, State(wsstate): State<WebSocketState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_toast(Extension(state.clone()), socket, wsstate))
}

pub async fn handle_toast(
    Extension(state): Extension<Arc<AppState>>, 
    socket: WebSocket, 
    wsstate: WebSocketState
) {
    let (ws_tx, mut ws_rx) = socket.split();
    let ws_tx = Arc::new(Mutex::new(ws_tx));

    let mut broadcast_rx = wsstate.broadcast_tx.lock().await.subscribe();
    
    tokio::spawn(async move {
        loop {
            tokio::select! {
                Some(result) = ws_rx.next() => {
                    match result {
                        Ok(msg) => {
                            match msg.into_text() {
                                Ok(info) => {
                                    if info == "keep-alive-toast" {
                                        println!("keep-alive recebido no handle_toast");
                                        continue; // Mantém o WebSocket aberto
                                    }

                                    println!("info: {}", info);
                                    
                                    // Tenta deserializar a mensagem
                                    let obj: Result<String, ParseError> = info.parse::<String>();
                                    match obj {
                                        Ok(obj) => {
                                            println!("handle_toast: {}", obj);                 

                                            let message = obj.into();  
                                            // Envia a mensagem para todos os WebSockets via broadcast
                                            if let Err(err) = wsstate.broadcast_tx.lock().await.send(Message::Text(message)) {
                                                eprintln!("Erro ao enviar mensagem de broadcast do handle_toast: {}", err);
                                                break;
                                            }
                                            println!("Mensagem enviada para todos os WebSockets pelo handle_toast.");

                                        }
                                        Err(err) => {
                                            eprintln!("Erro ao converter mensagem para String do handle_toast: {}", err);
                                            continue;
                                        }
                                    }
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
                Ok(broadcast_msg) = broadcast_rx.recv() => {
                    // Envia mensagens do canal de broadcast para o WebSocket conectado
                    if ws_tx.lock().await.send(broadcast_msg).await.is_err() {
                        eprintln!("Erro ao enviar mensagem de broadcast para o WebSocket");
                        break;
                    }
                }
                else => {
                    // Encerra o loop caso ambas as streams estejam fechadas
                    break;
                }
            }
        }
        println!("Conexão WebSocket encerrada.");
    });
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
        .route("/bang_ws", get(bang_handler))
        .route("/uso_carta_handler", get(uso_carta_handler))
        .route("/toast_ws", get(toast_handler))

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
