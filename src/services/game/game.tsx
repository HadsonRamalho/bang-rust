import { Personagem } from "@/interfaces/character/character";
import { client } from "..";
import { Jogo } from "../../interfaces/game/game";
import { Carta } from "@/interfaces/cards/cards";
import { Jogador } from "@/interfaces/player/player";

export async function iniciaJogo(nomes: string[]): Promise<Jogo> {
  try {
    const response = await client.post<Jogo>("/iniciar_jogo", {
      nomes
    });
    const data = response.data;
    return data;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao carregar o jogo: Código [${error.response?.status}]`
    );
  }
}

export async function listaPersonagens(): Promise<Personagem[]> {
  try {
    const response = await client.get<Personagem[]>("/lista_personagens");
    const data = response.data;
    return data;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao carregar os personagens: Código [${error.response?.status}]`
    );
  }
}

export async function usaCarta(carta: Carta, jogador: Jogador, jogo: Jogo){
  try {
    await client.post("/usa_carta", {
      carta, jogo, jogador
    });
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao carregar o jogo: Código [${error.response?.status}]`
    );
  }
}