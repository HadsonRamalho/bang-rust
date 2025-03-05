import { Carta } from "../cards/cards";
import { Jogador } from "../player/player";

export interface Jogo {
  jogadores: Jogador[],
  id: number,
  host: string,
  turno: string,
  logs: LogCarta[],
}

export interface LogCarta{
  nome_carta: string;
  nome_jogador: string;
  descricao: string;
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

export interface JogadorCartaAlvo{
  jogador: Jogador;
  idjogo: number;
  alvo: Jogador;
  carta: Carta;
}

export interface DescartaCarta{
  idjogo: number;
  jogador: Jogador;
  carta: Carta;
}