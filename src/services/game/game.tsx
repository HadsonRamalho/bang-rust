import { client } from "..";
import { Jogo } from "../../interfaces/game/game";

export async function startGame(nomes: string[]): Promise<Jogo> {
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
        `Falha ao listar as máquinas: Código [${error.response?.status}]`
      );
    }
  }