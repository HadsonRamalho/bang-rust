use std::sync::Arc;

use axum::{extract::ws::Utf8Bytes, Extension, Json};
use hyper::StatusCode;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

use super::{jogos::{atualiza_jogo, carrega_jogos, verifica_jogo_existe}, personagens::Personagem, AppState, Jogador, Jogo};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InfoCarta{
    pub nome: String,
    pub descricao: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Carta{
    Bang(InfoCarta),
    Esquiva(InfoCarta),
    Cerveja(InfoCarta),
    Saloon(InfoCarta)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogCarta{
    pub nome_carta: String,
    pub nome_jogador: String,
    pub descricao: String
}

impl Into<Utf8Bytes> for &LogCarta{
    fn into(self) -> Utf8Bytes {
        let log = &LogCarta { nome_carta: self.nome_carta.clone(), nome_jogador: self.nome_jogador.clone(), descricao: self.descricao.clone()};
        let log = log.to_owned();
        let json = serde_json::to_string(&log).unwrap();
        println!("json: {}", json);
        Utf8Bytes::from(json)
    }
}

#[derive(Serialize, Deserialize)]
pub struct JogadorJogo{
    jogador: Jogador,
    idjogo: u32
}

#[derive(Serialize, Deserialize)]
pub struct JogadorCartaAlvo{
    pub jogador: Jogador,
    pub idjogo: u32,
    pub alvo: Jogador,
    pub carta: Carta
}

impl Into<Utf8Bytes> for JogadorCartaAlvo{
    fn into(self) -> Utf8Bytes {
        let log = JogadorCartaAlvo{
            jogador: self.jogador,
            idjogo: self.idjogo,
            alvo: self.alvo,
            carta: self.carta,
        };
        let json = serde_json::to_string(&log).unwrap();
        println!("json: {}", json);
        Utf8Bytes::from(json)
    }
}


pub async fn lista_cartas()
    -> Json<Vec<Carta>>{
    let cartas = vec![
        Carta::Bang(InfoCarta{
            nome: "Bang".to_string(),
            descricao: "Serve como munição. Se quiser atirar em alguém, use uma carta “Bang!”.
             (Ela pode ser usada para se proteger contra a carta “Índios” também).".to_string()
        }),
        Carta::Esquiva(InfoCarta{
            nome: "Esquiva".to_string(),
            descricao: "Quando usarem um “Bang!” em você, jogue esta carta para não tomar o dano.
             Funciona contra a carta da “Metralhadora” também.".to_string()
        }),
        Carta::Cerveja(InfoCarta{
            nome: "Cerveja".to_string(),
            descricao: "Quando quiser recuperar um ponto de vida, beba uma “Cerveja” 
            (viu como Bang é um jogo fiel a realidade?). 
            Quando estiver com apenas 1 ponto de vida e receber 1 de dano, 
            se você tiver uma carta de Cerveja na mão, você a usa, e continua vivo com 1 de vida."
            .to_string()
        }),
        Carta::Saloon(InfoCarta{
            nome: "Saloon".to_string(),
            descricao: "Ao ser usada, paga-se uma rodada de cervejas para todos, 
            recuperando 1 ponto de vida de cada um dos jogadores ainda em jogo.".to_string()
        })
    ];

    Json(cartas)
}

pub async fn compra_cartas(Extension(state): Extension<Arc<AppState>>, input: Json<JogadorJogo>)
    -> Result<(StatusCode, Json<Vec<Carta>>), StatusCode>{
    
    if !verifica_jogo_existe(&state, input.idjogo).await{
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
    
    let mut jogos = carrega_jogos(&state).await;
    let jogo = jogos.iter_mut().find(|jogo| jogo.id == input.idjogo).unwrap();
    
    let limite = input.jogador.personagem.atributos.limitecompra;

    let limite_compra = limite;
    
    let cartas = lista_cartas().await.0;
    let cartas2 = lista_cartas().await.0;
    let cartas3 = lista_cartas().await.0;
    let mut cartasfinal = cartas.clone();
    cartasfinal.extend(cartas2);
    cartasfinal.extend(cartas3);

    let mut cartas = vec![];
    let mut rng = StdRng::from_os_rng();
    for i in 0..limite_compra.min(cartasfinal.len() as i32) {
        let index = rng.random_range(0..limite_compra);
        cartas.push(cartasfinal[index as usize].clone());
    }

    jogo.jogadores.iter_mut().filter(|p| p.nome == input.jogador.nome).for_each(|p| p.cartas.extend(cartas.clone()));

    jogo.logs.push(LogCarta{
        nome_carta: "Compra".to_string(),
        nome_jogador: input.jogador.nome.to_string(),
        descricao: format!("{} comprou {} cartas.", input.jogador.nome, cartas.len())
    });    

    {
        if let Some(jogo_mut) = state.jogos.lock().await.iter_mut().find(|j| j.id == input.idjogo) {
            *jogo_mut = jogo.to_owned();
        }
    }

    return Ok((StatusCode::OK, Json(cartas)))
}

#[derive(Serialize, Deserialize)]
pub struct DescartaCarta{
    pub idjogo: u32,
    pub jogador: Jogador,
    pub carta: Carta
}

impl Into<Utf8Bytes> for DescartaCarta{
    fn into(self) -> Utf8Bytes {
        let log = DescartaCarta{
            jogador: self.jogador,
            idjogo: self.idjogo,
            carta: self.carta,
        };
        let json = serde_json::to_string(&log).unwrap();
        println!("json: {}", json);
        Utf8Bytes::from(json)
    }
}


pub async fn descartar_carta(Extension(state): Extension<Arc<AppState>>,  input: Json<DescartaCarta>)
    -> Result<(StatusCode, Json<String>), StatusCode>{
    if !verifica_jogo_existe(&state, input.idjogo).await{
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    }

    let mut jogos = carrega_jogos(&state).await;
    let jogo = jogos.iter_mut().find(|jogo| jogo.id == input.idjogo).unwrap();

    let carta_descartada = input.carta.clone();

    let nome_carta = match carta_descartada{
        Carta::Bang(_) => "Bang",
        Carta::Esquiva(_) => "Esquiva",
        Carta::Cerveja(_) => "Cerveja",
        Carta::Saloon(_) => "Saloon"
    };

    if let Some(jogador) = jogo.jogadores.iter_mut().find(|p| p.nome == input.jogador.nome) {
        if let Some(pos) = jogador.cartas.iter().position(|c| c == &input.carta) {
            jogador.cartas.remove(pos);
        }
    }

    jogo.logs.push(LogCarta{
        nome_carta: "Descarte".to_string(),
        nome_jogador: input.jogador.nome.to_string(),
        descricao: format!("{} descartou a carta {}.", input.jogador.nome, nome_carta)
    });
    

    atualiza_jogo(&state, jogo.to_owned()).await;

    return Ok((StatusCode::OK, Json(nome_carta.to_string())))
}

pub async fn curar_personagem(Extension(state): Extension<Arc<AppState>>, input: Json<DescartaCarta>)
    -> Result<(StatusCode, Json<Jogo>), StatusCode>{
    if !verifica_jogo_existe(&state, input.idjogo).await{
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
    let mut jogos = carrega_jogos(&state).await;
    let jogo = jogos.iter_mut().find(|jogo| jogo.id == input.idjogo).unwrap();

    if let Some(jogador) = jogo.jogadores.iter_mut().find(|p| p.nome == input.jogador.nome) {
        if jogador.personagem.atributos.vida_atual < jogador.personagem.atributos.vida_maxima {
            jogador.personagem.atributos.vida_atual += 1;
        }
    }

    let nome_carta = match input.carta{
        Carta::Bang(_) => "Bang",
        Carta::Esquiva(_) => "Esquiva",
        Carta::Cerveja(_) => "Cerveja",
        Carta::Saloon(_) => "Saloon"
    };

    jogo.logs.push(LogCarta{
        nome_carta: nome_carta.to_string(),
        nome_jogador: input.jogador.nome.to_string(),
        descricao: format!("{} curou 1 ponto de vida.", input.jogador.nome)
    });

    atualiza_jogo(&state, jogo.to_owned()).await;

    return Ok((StatusCode::OK, Json(jogo.to_owned())))
}

pub async fn dano_bang(Extension(state): Extension<Arc<AppState>>, alvo: Json<JogadorJogo>)
    -> Result<(StatusCode, Json<Jogo>), StatusCode>{
    if !verifica_jogo_existe(&state, alvo.idjogo).await{
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let mut jogos = carrega_jogos(&state).await;
    let jogo = jogos.iter_mut().find(|jogo| jogo.id == alvo.idjogo).unwrap();

    let jogador = jogo.jogadores.iter_mut().find(|p| p.nome == alvo.jogador.nome).unwrap();
    jogador.personagem.atributos.vida_atual -= 1;

    let log = LogCarta{
        nome_carta: "Bang".to_string(),
        nome_jogador: alvo.jogador.nome.clone(),
        descricao: format!("{} tomou 1 de dano um Bang!", alvo.jogador.nome),
    };

    jogo.logs.push(log);

    atualiza_jogo(&state, jogo.to_owned()).await;

    return Ok((StatusCode::OK, Json(jogo.to_owned())))
}

pub async fn usar_bang_alvo(Extension(state): Extension<Arc<AppState>>, input: Json<JogadorCartaAlvo>)
    -> Result<(StatusCode, Json<Jogo>), StatusCode>{
    if !verifica_jogo_existe(&state, input.idjogo).await{
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    let mut jogos = carrega_jogos(&state).await;
    let jogo = jogos.iter_mut().find(|jogo| jogo.id == input.idjogo).unwrap();

    let mut nome_origem = input.jogador.nome.to_string();
    nome_origem.push_str(" -> ");
    nome_origem.push_str(&input.alvo.nome);


    let log = LogCarta{
        nome_carta: "Bang | WebSocket".to_string(),
        nome_jogador: nome_origem,
        descricao: format!("{} usou Bang em {}", input.jogador.nome, input.alvo.nome),
    };

    jogo.logs.push(log);

    atualiza_jogo(&state, jogo.to_owned()).await;

    return Ok((StatusCode::OK, Json(jogo.to_owned())))
}