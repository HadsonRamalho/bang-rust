use crate::core::cartas::{Carta, InfoCarta};
use crate::core::Jogo;
use crate::core::Jogador;
use axum::Json;
use serde::{Serialize, Deserialize};

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