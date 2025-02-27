export interface Personagem {
    nome: string,
    descricao: string,
    atributos: Atributos
}

export interface Atributos{
    vida_atual: number,
    vida_maxima: number,
    efeitos: string[],
    distancia: number,
    visao: number,
    limitecompra: number,
}
