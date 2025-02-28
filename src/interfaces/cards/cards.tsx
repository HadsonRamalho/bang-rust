export interface InfoCarta {
  nome: string,
  descricao: string
}

export type Carta =
  | { Bang: InfoCarta }
  | { Esquiva: InfoCarta }
  | { Cerveja: InfoCarta }
  | { Saloon: InfoCarta };
