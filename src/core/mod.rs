use axum::Json;
use rand::Rng;
use serde::{Deserialize, Serialize};

pub mod personagens;

#[derive(Debug, Serialize, Deserialize)]
pub struct Jogador{
    pub nome: String,
    pub funcao: Funcao,
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum TipoFuncao{
    Xerife,
    ForaDaLei,
    Vice,
    Renegado,
    Indefinido
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Funcao{
    pub nome: String,
    pub descricao: String,
    pub tipofuncao: TipoFuncao,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Jogo{
    pub jogadores: Vec<Jogador>
}

#[derive(Serialize, Deserialize)]
pub struct NomesJogadores{
    nomes: Vec<String>
}

pub fn choose_role(role_index: usize, has_xeriff: bool, has_vice: bool)
    -> Funcao{
    let roles = vec![
        Funcao{
            nome: "Xerife".to_string(),
            descricao: "Eliminar todos os Foras da Lei e o Renegado".to_string(),
            tipofuncao: TipoFuncao::Xerife
        },
        Funcao{
            nome: "Fora da Lei".to_string(),
            descricao: "Matar o Xerife".to_string(),
            tipofuncao: TipoFuncao::ForaDaLei
        },
        Funcao{
            nome: "Vice".to_string(),
            descricao: "Proteger o Xerife; Eliminar todos os Foras da Lei e o Renegado".to_string(),
            tipofuncao: TipoFuncao::Vice
        },
        Funcao{
            nome: "Renegado".to_string(),
            descricao: "Precisa matar o Xerife quando estiverem apenas os dois vivos na partida.".to_string(),
            tipofuncao: TipoFuncao::Renegado
        }
    ];

    let mut rng = rand::rng();

    if has_xeriff && role_index == 0{
        let index = rng.random_range(0..4);
        return choose_role(index, has_xeriff, has_vice);
    }

    if has_vice && role_index == 2{
        let index = rng.random_range(0..4);
        return choose_role(index, has_xeriff, has_vice);
    }

    return roles[role_index].clone()

}

pub async fn iniciar_jogo(input: Json<NomesJogadores>) -> Json<Jogo>{
    let nomes = input.nomes.clone();
    let num_players = nomes.len();

    if num_players < 4 || num_players > 8{
        println!("Número de jogadores inválido. O jogo deve ter entre 4 e 8 jogadores.");
        return Json(Jogo{
            jogadores: Vec::new()
        });
    }

    println!("Ok, o número de jogadores é {}", num_players);

    let mut players: Vec<Jogador> = Vec::new();

    for nome in nomes{
        let player = Jogador{
            nome,
            funcao: Funcao{
                nome: "Indefinido".to_string(),
                descricao: "Indefinido".to_string(),
                tipofuncao: TipoFuncao::Indefinido
            }
        };

        players.push(player);
    }

    let mut xerife_criado = false;
    let mut vice_criado = false;

    let mut rng = rand::rng();
    for player in players.iter_mut() {
        let mut has_role = false;
        while !has_role{
            let role_index = rng.random_range(0..4);
            let role = choose_role(role_index, xerife_criado, vice_criado);
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

    if !players.iter().any(|player| player.funcao.tipofuncao == TipoFuncao::Xerife){
        let mut rng = rand::rng();
        let index = rng.random_range(0..players.len());
        players[index].funcao = Funcao{
            nome: "Xerife".to_string(),
            descricao: "Eliminar todos os Foras da Lei e o Renegado".to_string(),
            tipofuncao: TipoFuncao::Xerife
        };
    }

    while !players.iter().any(|player| player.funcao.tipofuncao == TipoFuncao::Vice){
        let mut rng = rand::rng();
        let index = rng.random_range(0..players.len());
        if players[index].funcao.tipofuncao != TipoFuncao::Xerife{
            players[index].funcao = Funcao{
                nome: "Vice".to_string(),
                descricao: "Proteger o Xerife; Eliminar todos os Foras da Lei e o Renegado".to_string(),
                tipofuncao: TipoFuncao::Vice
            };
            break;
        }
    }

    let game = Jogo{
        jogadores: players
    };

    Json(game)
}