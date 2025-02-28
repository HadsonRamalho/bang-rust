use axum::Json;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct InfoCarta{
    pub nome: String,
    pub descricao: String
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Carta{
    Bang(InfoCarta),
    Esquiva(InfoCarta),
    Cerveja(InfoCarta),
    Saloon(InfoCarta)
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