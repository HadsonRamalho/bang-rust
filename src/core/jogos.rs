use std::sync::Arc;

use crate::core::cartas::{Carta, InfoCarta};
use crate::core::Jogo;
use crate::core::Jogador;
use axum::{Extension, Json};
use hyper::StatusCode;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Serialize, Deserialize};

use super::personagens::{self, lista_personagens, Personagem};
use super::{criar_baralho, escolher_cargo, AppState, Funcao, TipoFuncao};

#[derive(Serialize, Deserialize)]
pub struct CartaJogadorJogo{
    carta: Carta,
    jogo: Jogo,
    jogador: Jogador
}

pub async fn usa_carta(input: Json<CartaJogadorJogo>)
    -> Json<CartaJogadorJogo>{
    let carta = input.0.carta;
    let jogo = input.0.jogo;
    let jogador = input.0.jogador;

    println!("Jogador: {}", jogador.nome);
    match carta{
        Carta::Cerveja(_) => {
            println!("Cerveja");
        },
        Carta::Saloon(_) => {
            println!("Saloon");
        }
        Carta::Bang(_) => {
            println!("Bang!");
        },
        Carta::Esquiva(_) => {
            println!("Esquiva");
        }
    }

    Json(CartaJogadorJogo{
        carta,
        jogador,
        jogo
    })
}

#[derive(Serialize, Deserialize)]
pub struct EntrarJogo{
    pub nome: String,
    pub idjogo: u32
}

pub async fn verifica_jogo_existe(state: &Arc<AppState>, id: u32) -> bool{
    let jogo_existe = state.jogos.lock().await.iter().any(
        |jogo| jogo.id == id
    );
    return jogo_existe
}

pub async fn carrega_jogos(state: &Arc<AppState>) -> Vec<Jogo>{
    let jogos= state.jogos.lock().await.to_owned();
    jogos
}

#[axum::debug_handler]
pub async fn entrar_jogo(Extension(state): Extension<Arc<AppState>>, input: Json<EntrarJogo>)
    -> Result<(StatusCode, Json<Jogo>), StatusCode>{
    {
        let id = input.idjogo;
        let jogo_existe = verifica_jogo_existe(&state, id).await;

        if !jogo_existe{
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    let mut jogos = carrega_jogos(&state).await;
    let jogo = jogos.iter_mut().find(|jogo| jogo.id == input.idjogo).unwrap();

    let novo_player = Jogador{
        nome: input.nome.to_string(),
        funcao: Funcao{
            nome: "Indefinido".to_string(),
            descricao: "Indefinido".to_string(),
            tipofuncao: TipoFuncao::Indefinido
        },
        personagem: Personagem{
            nome: "Indefinido".to_string(),
            descricao: "Indefinido".to_string(),
            atributos: personagens::Atributos{
                vida_atual: 0,
                vida_maxima: 0,
                efeitos: Vec::new(),
                distancia: 0,
                visao: 0,
                limitecompra: 0
            }
        },
        cartas: vec![]
    };

    jogo.jogadores.push(novo_player);

    let mut xerife_criado = jogo.jogadores.iter().any(|jogador|
    jogador.funcao.tipofuncao == TipoFuncao::Xerife);
    let mut vice_criado = jogo.jogadores.iter().any(|jogador|
        jogador.funcao.tipofuncao == TipoFuncao::Vice);

    let mut rng = StdRng::from_os_rng();
    for player in jogo.jogadores.iter_mut() {
        let mut has_role = player.funcao.nome != "Indefinido";
        while !has_role{
            let role_index = rng.random_range(0..4);
            let role = escolher_cargo(role_index, xerife_criado, vice_criado);
            if role.tipofuncao == TipoFuncao::Xerife{
                xerife_criado = true;
            }
            if role.tipofuncao == TipoFuncao::Vice{
                vice_criado = true;
            }

            player.funcao = role;
            
            has_role = true;
        }
    }

    if !jogo.jogadores.iter().any(|player| player.funcao.tipofuncao == TipoFuncao::Xerife){
        let mut rng = StdRng::from_os_rng();
        let index = rng.random_range(0..jogo.jogadores.len());
        jogo.jogadores[index].funcao = Funcao{
            nome: "Xerife".to_string(),
            descricao: "Eliminar todos os Foras da Lei e o Renegado".to_string(),
            tipofuncao: TipoFuncao::Xerife
        };
    }

    while !jogo.jogadores.iter().any(|player| player.funcao.tipofuncao == TipoFuncao::Vice){
        let mut rng = StdRng::from_os_rng();
        let index = rng.random_range(0..jogo.jogadores.len());
        if jogo.jogadores[index].funcao.tipofuncao != TipoFuncao::Xerife{
            jogo.jogadores[index].funcao = Funcao{
                nome: "Vice".to_string(),
                descricao: "Proteger o Xerife; Eliminar todos os Foras da Lei e o Renegado".to_string(),
                tipofuncao: TipoFuncao::Vice
            };
            break;
        }
    }

    let personagens = lista_personagens().await.0;

    let mut repetido = false;
    while jogo.jogadores.iter().any(|p| p.personagem.nome == "Indefinido")
    || repetido{
        let mut rng = StdRng::from_os_rng();
        let personagem_index = rng.random_range(0..personagens.len());
        let personagem = personagens[personagem_index].clone();
        let mut rng = StdRng::from_os_rng();
        let index = rng.random_range(0..jogo.jogadores.len());
        if jogo.jogadores.iter().any(|p| p.personagem.nome == personagem.nome){
            repetido = true;
        }
        if !(jogo.jogadores.iter().any(|p| p.personagem.nome == personagem.nome)){
            jogo.jogadores[index].personagem = personagem;
            repetido = false;
        }
    }

    for player in jogo.jogadores.iter_mut(){
        if player.cartas.len() == 0{
            println!("Nome do jogador: {}", player.nome);
            player.cartas = criar_baralho(player.personagem.atributos.vida_maxima as usize).await;
        }
    }

    {
        if let Some(jogo_mut) = state.jogos.lock().await.iter_mut().find(|j| j.id == input.idjogo) {
            *jogo_mut = jogo.to_owned();
        }
    }
    
    let jogo = carrega_jogos(&state).await.iter().find(|jogo|
        jogo.id == input.idjogo).unwrap().to_owned();

    let jogador_existe = carrega_jogos(&state).await.iter().any(|jogo|
            jogo.jogadores.iter().any(|jogador| jogador.nome == input.nome));
    if !jogador_existe{
        println!("O jogador {} não foi encontrado", input.nome);
        return Err(StatusCode::BAD_REQUEST)
    }
    println!("O jogador {} foi encontrado", input.nome);
    Ok((StatusCode::OK, Json(jogo)))
}

pub async fn carregar_jogo(Extension(state): Extension<Arc<AppState>>, input: Json<EntrarJogo>)
    -> Result<(StatusCode, Json<Jogo>), StatusCode>{
    {
        let id = input.idjogo;
        let jogo_existe = verifica_jogo_existe(&state, id).await;

        if !jogo_existe{
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    let mut jogos = carrega_jogos(&state).await;
    let jogo = jogos.iter_mut().find(|jogo| jogo.id == input.idjogo).unwrap();
    return Ok((StatusCode::OK, Json(jogo.to_owned())))
}