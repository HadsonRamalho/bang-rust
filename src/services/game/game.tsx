import { Personagem } from "@/interfaces/character/character";
import { client } from "..";
import { Jogo, ResUsoCarta } from "../../interfaces/game/game";
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

export async function usaCarta(carta: Carta, jogador: Jogador, jogo: Jogo): Promise<ResUsoCarta> {
  try {
    const res = await client.post<ResUsoCarta>("/usa_carta", {
      carta, jogo, jogador
    });
    const data: ResUsoCarta = res.data;
    return data;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao usar a carta: Código [${error.response?.status}]`
    );
  }
}

export async function compraCartas(qtd: number): Promise<Carta[]> {
  try {
    const res = await client.post<Carta[]>("/compra_cartas", {
      qtd
    });
    const data: Carta[] = res.data;
    return data;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao comprar cartas: Código [${error.response?.status}]`
    );
  }
}