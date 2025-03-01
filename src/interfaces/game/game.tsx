import { Carta } from "../cards/cards";
import { Jogador } from "../player/player";

export interface Jogo {
  jogadores: Jogador[],
  id: number,
  host: string,
  turno: string
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

export interface EntrarJogo{
  nome: string;
  idjogo: number;
}