import { useEffect, useState } from "react";
import "./App.css";
import { Jogador } from "./interfaces/player/player";
import { Jogo } from "./interfaces/game/game";
import { Bullet } from "./components/bullet";
import { Button } from "./components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { iniciaJogo, listaPersonagens } from "./services/game/game";
import { Personagem } from "./interfaces/character/character";
import { Avatar } from "./components/ui/avatar";
import { CardSvgIcon } from "./components/card-svg-icon";

function App() {
  const [players, setPlayers] = useState<Jogador[]>([]);
  const [personagens, setPersonagens] = useState<Personagem[]>([]);

  const [qtdPlayers, setQtdPlayers] = useState(0);

  const [playerNames, setPlayerNames] = useState<string[]>(
    Array(qtdPlayers).fill("")
  );

  const loadGame = async () => {
    const res: Jogo = await iniciaJogo(playerNames);
    console.log(res);
    setPlayers(res.jogadores);
  };

  const loadCharacters = async () => {
    const res: Personagem[] = await listaPersonagens();
    console.log(res);
    setPersonagens(res);
  };

  useEffect(() => {
    loadCharacters();
  }, []);

  return (
    <>
      <div>
        <div className="w-full">
          <Tabs defaultValue="Jogadores" className="w-full">
            <TabsList className="grid w-full grid-cols-2 bg-[hsl(var(--orange-1))]">
              <TabsTrigger
                value="Jogadores"
                className="hover:cursor-pointer focus:bg-[hsl(var(--primary))]"
              >
                Jogadores
              </TabsTrigger>
              <TabsTrigger
                value="personagens"
                className="hover:cursor-pointer focus:bg-[hsl(var(--primary))]"
              >
                Personagens
              </TabsTrigger>
            </TabsList>
            <TabsContent value="Jogadores">
              <Card className="border-[hsl(var(--primary))]">
                <CardHeader>
                  <CardTitle>Iniciar Jogo</CardTitle>
                  <CardDescription>Testano</CardDescription>
                </CardHeader>
                <CardContent className="space-y-2">
                  <div>
                    <Label htmlFor="qtdplayers" className="mb-2">
                      Quantidade de jogadores
                    </Label>
                    <Input
                      id="qtdplayers"
                      className="focus:ring-[hsl(var(--primary))] border-[hsl(var(--primary))]"
                      max={7}
                      min={4}
                      onChange={(e) => setQtdPlayers(parseInt(e.target.value))}
                    />
                  </div>

                  {qtdPlayers > 3 && qtdPlayers < 8 && (
                    <div>
                      {Array.from({ length: qtdPlayers }).map((_, index) => (
                        <div key={index}>
                          <Label htmlFor={`player${index}`} className="mb-2">
                            Nome do jogador {index + 1}
                          </Label>
                          <Input
                            className="border-[hsl(var(--primary))] focus:ring-[hsl(var(--primary))]"
                            id={`player${index}`}
                            onChange={(e) => {
                              const newPlayerNames = [...playerNames];
                              newPlayerNames[index] = e.target.value;
                              setPlayerNames(newPlayerNames);
                            }}
                          />
                        </div>
                      ))}
                    </div>
                  )}
                </CardContent>
                <CardFooter className="flex justify-center">
                  <Button
                    onClick={async () => {
                      await loadGame();
                    }}
                    className="hover:cursor-pointer focus:bg-[hsl(var(--primary))]
                  bg-[hsl(var(--primary))] text-[hsl(var(--text))]"
                  >
                    Gerar funções
                  </Button>
                </CardFooter>
                <CardContent>
                  <div>
                    {players.length > 0 && (
                      <span>
                        <strong>Jogadores:</strong>
                      </span>
                    )}
                    {players.map((player, index) => (
                      <Card
                        className="border-[hsl(var(--primary))] space-y-2 mb-2"
                        key={index}
                      >
                        <CardHeader>
                          <p>
                            <strong>Nome:</strong> {player.nome}
                          </p>

                          <CardHeader className="flex items-center space-x-4">
                            <Avatar className="w-20 h-20">
                              <img src={`${player.funcao.nome}.png`} alt="" />
                            </Avatar>
                          </CardHeader>

                          <p>
                            <strong>Função: </strong> {player.funcao.nome}
                          </p>
                        </CardHeader>
                        <CardContent>
                          <div>
                            <p>
                              <strong>Personagem:</strong>{" "}
                              {player.personagem.nome}
                            </p>
                            <p>
                              <strong>Descrição:</strong>{" "}
                              {player.personagem.descricao}
                            </p>
                            <div>
                              <strong>
                                <p>Vida:</p>
                              </strong>
                              <div className="flex space-x-1 mb-4">
                                {Array.from({
                                  length:
                                    player.personagem.atributos.vida_maxima,
                                }).map((_, index) => (
                                  <span key={index}>
                                    <Bullet
                                      color="hsl(var(--primary))"
                                      size={35}
                                    ></Bullet>
                                  </span>
                                ))}
                              </div>
                              {player.cartas.map((carta, index) => {
                                const [tipo, info] = Object.entries(carta)[0]; // Pega a chave e o valor

                                return (
                                  <Card className="mb-2 border-[hsl(var(--primary))]" key={index}>
                                    <CardHeader className="flex items-center space-x-0">
                                      <CardSvgIcon tipo={tipo} size={45} />
                                      <p><strong>{tipo}</strong></p>
                                    </CardHeader>
                                    <CardContent>
                                      {info.descricao}
                                    </CardContent>
                                  </Card>
                                );
                              })}
                            </div>
                          </div>
                        </CardContent>
                      </Card>
                    ))}
                  </div>
                </CardContent>
              </Card>
            </TabsContent>
            <TabsContent value="personagens">
              <Card className="border-[hsl(var(--primary))]">
                <CardHeader>
                  <CardTitle>Personagens</CardTitle>
                  <CardDescription>Lista de Personagens</CardDescription>
                </CardHeader>
                <CardContent className="space-y-2">
                  {personagens.map((personagem, index) => (
                    <Card
                      className="border-[hsl(var(--primary))] space-y-2 mb-2"
                      key={index}
                    >
                      <CardHeader className="flex items-center space-x-4">
                        <Avatar className="w-20 h-20">
                          <img src={`${personagem.nome}.png`} alt="" />
                        </Avatar>
                        <p>
                          <strong>Nome: </strong>
                          {personagem.nome}
                        </p>
                        <p>
                          <strong>Descrição:</strong>
                          {personagem.descricao}
                        </p>
                        <Card className="border-[hsl(var(--primary))]">
                          <CardHeader>
                            <strong>Atributos</strong>
                          </CardHeader>
                          <CardContent>
                            <p>
                              <strong>Vida Máxima: </strong>
                              {personagem.atributos.vida_maxima}
                            </p>
                            <p>
                              <strong>Alcance da Visão: </strong>
                              {personagem.atributos.visao}
                            </p>
                            <p>
                              <strong>Distância: </strong>
                              {personagem.atributos.distancia}
                            </p>
                          </CardContent>
                        </Card>
                      </CardHeader>
                    </Card>
                  ))}
                </CardContent>
              </Card>
            </TabsContent>
          </Tabs>
        </div>
      </div>
    </>
  );
}

export default App;
