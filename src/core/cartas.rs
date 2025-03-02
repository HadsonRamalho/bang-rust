use std::sync::Arc;

use axum::{Extension, Json};
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

#[derive(Serialize, Deserialize, Clone)]
pub struct LogCarta{
    pub nome_carta: String,
    pub nome_jogador: String,
    pub descricao: String
}

#[derive(Serialize, Deserialize)]
pub struct JogadorJogo{
    jogador: Jogador,
    idjogo: u32
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