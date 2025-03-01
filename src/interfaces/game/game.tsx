import { Carta } from "../cards/cards";
import { Jogador } from "../player/player";

export interface Jogo {
  jogadores: Jogador[],
  id: number
}

export interface ResUsoCarta{
  carta: Carta;
  jogador: Jogador;
  jogo: Jogo;
}

export interface LogsCartas{
  nomeCarta: string,
  nomeJogador: string,
  descricao: string
}