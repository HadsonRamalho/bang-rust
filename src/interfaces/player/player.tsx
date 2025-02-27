import { Personagem } from "../character/character";
import { Funcao } from "../role/role";

export interface Jogador {
  nome: string,
  funcao: Funcao,
  personagem: Personagem
}
  