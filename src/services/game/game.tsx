import { Personagem } from "@/interfaces/character/character";
import { client } from "..";
import { EntrarJogo, Jogo, ResUsoCarta } from "../../interfaces/game/game";
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

export async function compraCartas(jogador: Jogador, idjogo: number): Promise<Carta[]> {
  try {
    const res = await client.post<Carta[]>("/compra_cartas", {
      jogador: jogador,
      idjogo: idjogo
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

export async function carregaJogo(entrarJogo: EntrarJogo): Promise<Jogo> {
  try {
    const res = await client.post<Jogo>('https://g6v9psc0-3069.brs.devtunnels.ms/carregar_jogo', {
      nome: entrarJogo.nome,
      idjogo: entrarJogo.idjogo,
    });
    const data: Jogo = res.data;
    return data;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao carregar o jogo: Código [${error.response?.status}]`
    );
  }
}

export async function entraJogo(entrarJogo: EntrarJogo): Promise<Jogo> {
  try {
    const res = await client.post<Jogo>('https://g6v9psc0-3069.brs.devtunnels.ms/entrar_jogo', {
      nome: entrarJogo.nome,
      idjogo: entrarJogo.idjogo,
    });
    const data: Jogo = res.data;
    return data;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao carregar o jogo: Código [${error.response?.status}]`
    );
  }
}

export async function passaTurno(entrarJogo: EntrarJogo): Promise<Jogo> {
  try {
    const res = await client.post<Jogo>('https://g6v9psc0-3069.brs.devtunnels.ms/passar_turno', {
      nome: entrarJogo.nome,
      idjogo: entrarJogo.idjogo,
    });
    const data: Jogo = res.data;
    return data;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao passar o turno: Código [${error.response?.status}]`
    );
  }
}

export async function descartaCarta(jogador: Jogador, idjogo: number, carta: Carta): Promise<string> {
  try {
    const res = await client.post<string>("/descartar_carta", {
      jogador: jogador,
      idjogo: idjogo,
      carta: carta
    });
    const data: string = res.data;
    return data;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao descartar carta: Código [${error.response?.status}]`
    );
  }
}

export async function curaPersonagem(jogador: Jogador, idjogo: number, carta: Carta): Promise<Jogo> {
  try {
    const res = await client.post<Jogo>("/curar_personagem", {
      jogador: jogador,
      idjogo: idjogo,
      carta: carta
    });
    const data: Jogo = res.data;
    return data;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao curar: Código [${error.response?.status}]`
    );
  }
}

export async function danoBang(jogador: Jogador, idjogo: number): Promise<Jogo> {
  try {
    const res = await client.post<Jogo>("/dano_bang", {
      jogador: jogador,
      idjogo: idjogo
    });
    const data: Jogo = res.data;
    return data;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.error(error.response?.status, error.message);
    throw new Error(
      `Falha ao causar dano do bang: Código [${error.response?.status}]`
    );
  }
}