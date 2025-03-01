import { useEffect, useRef, useState } from "react";
import "./App.css";
import type { Jogador } from "./interfaces/player/player";
import type { CarregarJogoType, EntrarJogo, Jogo, LogsCartas } from "./interfaces/game/game";
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
import { carregaJogo, compraCartas, entraJogo, iniciaJogo, listaPersonagens, passaTurno, usaCarta } from "./services/game/game";
import type { Personagem } from "./interfaces/character/character";
import { Avatar } from "./components/ui/avatar";
import { CardSvgIcon } from "./components/card-svg-icon";
import { useMask } from "@react-input/mask";
import { toast } from "sonner";
import type { Carta } from "./interfaces/cards/cards";
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "@/components/ui/sheet";
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import DetalhesCarta from "./components/detalhes-carta";

function App() {
  const [players, setPlayers] = useState<Jogador[]>([]);
  const [personagens, setPersonagens] = useState<Personagem[]>([]);

  const [logs, setLogs] = useState<LogsCartas[]>([]);

  const [turno, setTurno] = useState(
    players.find((player) => player.funcao.nome === "Xerife")?.nome,
  );

  const [qtdPlayers, setQtdPlayers] = useState(0);
  const [jogo, setJogo] = useState<Jogo>();

  const [idjogo, setidjogo] = useState<number>();

  const [playerNames, setPlayerNames] = useState<string[]>(
    Array(qtdPlayers).fill(""),
  );

  const loadGame = async () => {
    setLogs([]);
    const res: Jogo = await iniciaJogo(playerNames);
    if (res.id === 0) {
      return;
    }
    const novoLog: LogsCartas = {
      nomeCarta: "Sistema",
      descricao: "O jogo iniciou!",
      nomeJogador: "Sistema",
    };
    setLogs((prevLogs) => [...prevLogs, novoLog]);
    const turnoAtual = res.jogadores.find(
      (player) => player.funcao.nome === "Xerife",
    )?.nome;
    setTurno(turnoAtual);
    toast(`${turnoAtual} é o Xerife!`, {
      style: {
        backgroundColor: "hsl(var(--orange-1))",
      },
    });
    const logXerife: LogsCartas = {
      nomeCarta: "Sistema",
      descricao: `${turnoAtual} é o Xerife!`,
      nomeJogador: "Sistema",
    };
    setLogs((prevLogs) => [...prevLogs, logXerife]);
    setPlayers(res.jogadores);
    console.log("idjogo: ", res.id);
    setJogo(res);
    setTurno(res.turno);
    const ws = new WebSocket('wss://g6v9psc0-3069.brs.devtunnels.ms/listar_handler');

    ws.onopen = () => {
      console.log('Conectado ao WebSocket de Listagem do LoadGame');
      ws.send('novo jogo criado');
    };

    ws.onmessage = (event) => {
      const newMessage = event.data;
      setIdsJogos(newMessage);
      console.log("mensagem: ", newMessage);
    };

    ws.onerror = (error) => {
      console.error('Erro no WebSocket do LoadGame:', error);
    };

    ws.onclose = () => {
      console.log('Desconectado do WebSocket de Listagem do LoadGame');
    };

    return () => {
      ws.close();
    };

  };

  const loadCharacters = async () => {
    const res: Personagem[] = await listaPersonagens();
    console.log(res);
    setPersonagens(res);
  };

  useEffect(() => {
    loadCharacters();
  }, []);

  const usarCarta = async (carta: Carta, jogador: Jogador, jogo: Jogo) => {
    console.log(carta);
    const res = await usaCarta(carta, jogador, jogo);
    if (res.jogador.nome) {
      const [tipo, descricao] = Object.entries(res.carta)[0];
      if (tipo === "Saloon") {
        players.map((player) => {
          if (player.personagem.atributos.vida_atual < player.personagem.atributos.vida_maxima) {
            const novaVida = Math.min(player.personagem.atributos.vida_atual + 1, player.personagem.atributos.vida_maxima);
            player.personagem.atributos.vida_atual = novaVida;
            console.log(`${player.nome} foi curado pelo Saloon de ${jogador.nome}`);
          }
        })
          ;
      }
      const log: LogsCartas = {
        nomeCarta: descricao.nome,
        nomeJogador: jogador.nome,
        descricao: descricao.descricao,
      };
      setLogs((prevLogs) => [...prevLogs, log]);
      toast(`${res.jogador.nome} usou ${tipo}!`, {
        style: {
          backgroundColor: "hsl(var(--orange-1))",
        },
      });
    }
  };

  const comprarCartas = async (jogador: Jogador) => {
    console.log("jogador: ", jogador);
    if (jogador.personagem.atributos.vida_atual < jogador.personagem.atributos.limitecompra) {
      const cartas = await compraCartas(jogador.personagem.atributos.vida_atual);
      console.log("Cartas compradas: ", cartas);
      return cartas;
    }
    const cartas = await compraCartas(jogador.personagem.atributos.limitecompra);
    console.log("Cartas compradas: ", cartas);
    return cartas
  }

  const descartarCarta = (carta: Carta, index: number, nomePlayer: string, nomeCarta: string) => {
    toast(`${nomePlayer} descartou ${nomeCarta}`);
    setPlayers(prevPlayers => {
      const updatedPlayers = [...prevPlayers];
      updatedPlayers[index] = {
        ...updatedPlayers[index],
        cartas: updatedPlayers[index].cartas.filter(c => c !== carta)
      };
      return updatedPlayers;
    });
  };

  const passarTurno = async (nome: string) => {
    console.log(idjogo);
    if(idjogo){
      console.log("setando");
      const jogoTurno = await passaTurno({
        nome: nome,
        idjogo: idjogo
      });
      setJogo(jogoTurno);
      setTurno(jogoTurno.turno);
      console.log("Jogo setado: ", jogoTurno);
    }
  }

  const inputRef = useMask({
    mask: "_",
    replacement: { _: /\d/ },
  });

  const inputRefEntrar = useMask({
    mask: "_________",
    replacement: { _: /\d/ },
  });

  const [messages, setMessages] = useState<string[]>([]);
  const [inputMessage, setInputMessage] = useState('');
  const [socket, setSocket] = useState<WebSocket | null>(null);


  const conectarJogo = () => {
    const ws = new WebSocket('wss://g6v9psc0-3069.brs.devtunnels.ms/ws');

    ws.onopen = () => {
      console.log('Conectado ao WebSocket');
      setSocket(ws);
    };

    ws.onmessage = (event) => {
      const newMessage = event.data;
      setMessages((prevMessages) => [...prevMessages, newMessage]);
    };

    ws.onclose = () => {
      console.log('Desconectado do WebSocket');
      setSocket(null);
    };

    return () => {
      ws.close();
    };
  }

  const [idsJogos, setIdsJogos] = useState<string>();
  const myws = useRef<WebSocket | null>(null);

  useEffect(() => {
    const connect = () => {
      myws.current = new WebSocket('wss://g6v9psc0-3069.brs.devtunnels.ms/listar_handler');

      myws.current.onopen = () => {
        console.log('Conectado ao WebSocket de Listagem');
        myws.current?.send('nova sessão iniciada');

        // Enviar mensagens de keep-alive periodicamente
        const keepAliveInterval = setInterval(() => {
          if (myws.current?.readyState === WebSocket.OPEN) {
            myws.current.send('keep-alive');
          }
        }, 30000); // Envia a cada 30 segundos

        return () => clearInterval(keepAliveInterval);
      };

      myws.current.onmessage = (event) => {
        const newMessage = event.data;
        setIdsJogos(newMessage);
      };

      myws.current.onerror = (error) => {
        console.error('Erro no WebSocket:', error);
      };

      myws.current.onclose = () => {
        console.log('Desconectado do WebSocket de Listagem');

        // Tentar reconectar após 3 segundos
        setTimeout(() => {
          console.log('Tentando reconectar ao WebSocket...');
          connect();
        }, 3000);
      };
    };

    connect();

    // Fechar o WebSocket ao desmontar o componente
    return () => {
      myws.current?.close();
    };
  }, []);

  useEffect(() => {
    conectarJogo();
  }, []);

  const [nome, setNome] = useState("")
  const carregarJogo = async () => {
    if (!idjogo || !nome) {
      return;
    }
    const obj: EntrarJogo = {
      nome: nome,
      idjogo: idjogo
    };

    const jogoCarregadoVerificacao = await carregaJogo(obj);

    const jogadorExistente = jogoCarregadoVerificacao.jogadores.some((jogador) => jogador.nome === nome);

    if (jogadorExistente) {
      console.log("Jogador já está na partida");
      setJogo(jogoCarregadoVerificacao);
      setPlayers(jogoCarregadoVerificacao.jogadores);
      setTurno(jogoCarregadoVerificacao.turno);
      return;
    }

    console.log("Jogador não encontrado, entrando no jogo");

    const entrouJogo = await entraJogo(obj);
    console.log("Entrou no jogo: ", entrouJogo);

    const jogoCarregado = await carregaJogo(obj);
    console.log("Jogo carregado após entrar: ", jogoCarregado);

    setJogo(jogoCarregado);
    setPlayers(jogoCarregado.jogadores);
    setTurno(jogoCarregado.turno);
  };


  const sendMessage = () => {
    if (socket) {
      socket.send(inputMessage);
      console.log("tentou enviar");
    }
  };

  return (
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
          <Card className="mb-2 border-[hsl(var(--primary))]">
            <CardHeader>
              Salas disponíveis
            </CardHeader>
            <CardContent>
              {idsJogos ? (
                idsJogos.split(';').map((numero) => (
                  <p className="border-[2px] border-[hsl(var(--primary))] mb-2" key={numero.trim()}>{numero.trim()}</p>
                ))
              ) : (
                <>Nenhum jogo disponível</>
              )}
            </CardContent>
          </Card>
          <Card className="mb-2 mt-2 border-[hsl(var(--primary))]">
            <CardTitle className="text-xl">Entrar em jogo</CardTitle>
            <CardContent>
              <Input
                className="focus:ring-[hsl(var(--primary))] border-[hsl(var(--primary))] mb-2"
                placeholder="Seu nome"
                onChange={(e) => { setNome(e.target.value) }}
              />
              <Input
                className="focus:ring-[hsl(var(--primary))] border-[hsl(var(--primary))]"
                max={11111}
                min={9999999}
                ref={inputRefEntrar}
                placeholder="ID do jogo"
                onBlur={(e) => {
                  if (
                    Number.parseInt(e.target.value) > 11111 &&
                    Number.parseInt(e.target.value) < 9999999
                  ) {
                    setidjogo(Number.parseInt(e.target.value));

                    return;
                  }
                  alert("A quantidade de jogadores é inválida.");
                }}
              />
              <Button onClick={carregarJogo} className="bg-[hsl(var(--primary))] mt-2">Entrar</Button>
            </CardContent>
          </Card>
          <Card className="border-[hsl(var(--primary))]">
            {jogo ? (
              <CardHeader>
                <CardTitle className="text-xl">
                  Partida de <strong>{jogo.host}</strong>
                  <p className="mt-2">Lista de Jogadores:</p>
                  {jogo.jogadores.map((jogador) => (
                    <p key={jogador.nome}>{jogador.nome === jogo.host ?
                      (<strong>{jogador.nome}</strong>)
                      : (<>{jogador.nome}</>)
                    }</p>
                  ))}
                </CardTitle>
                <CardDescription>
                  {jogo?.id && (
                    <p className="text-xl">ID do jogo: <strong> {jogo.id}</strong></p>
                  )}
                </CardDescription>
              </CardHeader>
            ) : (
              <CardHeader>
                <CardTitle className="text-xl">Novo Jogo</CardTitle>
              </CardHeader>
            )}
            <CardContent className="space-y-2">
              {(!qtdPlayers && !jogo) && (
                <>
                  <Label htmlFor="qtdplayers" className="mb-2">
                    Quantidade de jogadores
                  </Label>
                  <Input
                    id="qtdplayers"
                    className="focus:ring-[hsl(var(--primary))] border-[hsl(var(--primary))]"
                    max={7}
                    min={4}
                    ref={inputRef}
                    onBlur={(e) => {
                      if (
                        Number.parseInt(e.target.value) > 3 &&
                        Number.parseInt(e.target.value) < 8
                      ) {
                        setQtdPlayers(Number.parseInt(e.target.value));
                        return;
                      }
                      alert("A quantidade de jogadores é inválida.");
                    }}
                  />
                </>
              )}
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
              <div>
                <h1>WebSocket Chat</h1>
                <div>
                  {messages.map((message, index) => (
                    <div key={index}>{message}</div>
                  ))}
                </div>
                <input
                  type="text"
                  value={inputMessage}
                  onChange={(e) => setInputMessage(e.target.value)}
                />
                <button type="button" onClick={sendMessage}>Enviar</button>
              </div>
              <Sheet>
                <SheetTrigger>
                  <Button className="bg-[hsl(var(--primary))] hover:cursor-pointer">
                    Ver histórico do jogo
                  </Button>
                </SheetTrigger>
                <SheetContent className="bg-gray-200">
                  <SheetHeader>
                    <SheetTitle>Histórico do Jogo</SheetTitle>
                    <SheetDescription>
                      <Table>
                        <TableHeader>
                          <TableRow>
                            <TableHead>Origem</TableHead>
                            <TableHead>Carta</TableHead>
                            <TableHead>Descrição</TableHead>
                          </TableRow>
                        </TableHeader>
                        <TableBody>
                          {logs.length > 0 ? (
                            logs.map((log, index) => (
                              <TableRow
                                key={`${log.nomeJogador}-${log.nomeCarta}-${index}`}
                              >
                                <TableCell className="font-medium">
                                  {log.nomeJogador}
                                </TableCell>
                                <TableCell>{log.nomeCarta}</TableCell>
                                <TableCell>
                                  <Button
                                    className="bg-[hsl(var(--primary))] hover:cursor-pointer"
                                    onClick={() => {
                                      toast(log.descricao);
                                    }}
                                  >
                                    Ver descrição
                                  </Button>
                                </TableCell>
                              </TableRow>
                            ))
                          ) : (
                            <p>O jogo ainda não iniciou!</p>
                          )}
                        </TableBody>
                      </Table>
                    </SheetDescription>
                  </SheetHeader>
                </SheetContent>
              </Sheet>
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
              {players.length > 0 && (
                <span>
                  <strong>Jogadores:</strong>
                </span>
              )}
              {players.map((player, indexPlayer) => (
                <Card
                  key={indexPlayer}
                  className={
                    turno === player.nome
                      ? "border-[hsl(var(--primary))] border-[4px] space-y-2 mb-2"
                      : "border-[hsl(var(--primary))] space-y-1 mb-1"
                  }
                >
                  <p>
                    <strong>Nome:</strong> {player.nome}
                  </p>
                  {(player.nome === nome || player.funcao.nome === "Xerife") && (

                    <CardHeader>
                      <CardHeader className="flex items-center space-x-4">

                        <Avatar className="w-20 h-20">
                          <img src={`${player.funcao.nome}.png`} alt="" />
                        </Avatar>
                      </CardHeader>

                      <p>
                        {" "}
                        <strong>Função: </strong> {player.funcao.nome}{" "}
                      </p>
                    </CardHeader>
                  )}
                  <CardHeader className="flex items-center space-x-4">

                    <Avatar className="w-20 h-20">
                      <img src={`${player.personagem.nome}.png`} alt="" />
                    </Avatar>
                  </CardHeader>

                  <CardContent>
                    <p>
                      {" "}
                      <strong>Personagem:</strong> {player.personagem.nome}{" "}
                    </p>
                    <p>
                      {" "}
                      <strong>Descrição:</strong> {
                        player.personagem.descricao
                      }{" "}
                    </p>
                    <div>
                      <strong>
                        {" "}
                        <p>Vida:</p>{" "}
                      </strong>
                      <div className="flex space-x-1 mb-4">
                        {Array.from({
                          length: player.personagem.atributos.vida_maxima,
                        }).map((_, index) => (
                          <span key={index}>
                            <Bullet color="hsl(var(--primary))" size={35} />
                          </span>
                        ))}
                      </div>
                        
                      {player.nome === nome && (
                        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">

    {player.cartas.map((carta, index) => {
                          const [tipo, info] = Object.entries(carta)[0];
                          return (
                            <Card
                              className="mb-2 border-[hsl(var(--primary))]"
                              key={index}
                            >
                              <CardHeader className="flex items-center space-x-0">
                                <CardSvgIcon tipo={tipo} size={45} />
                                <p>
                                  <strong>{tipo}</strong>
                                </p>
                              </CardHeader>
                              <CardFooter className="flex flex-col items-center gap-2">
                                {turno === player.nome && player.nome === nome && tipo !== "Esquiva" && (
                                  <Button
                                    onClick={async () => {
                                      if (jogo) {
                                        await usarCarta(carta, player, jogo);
                                        player.cartas = player.cartas.filter((c) => c !== carta);
                                      }
                                    }}
                                    disabled={
                                      player.personagem.atributos.vida_atual ===
                                      player.personagem.atributos.vida_maxima &&
                                      tipo === "Cerveja"
                                    }
                                    className="bg-[hsl(var(--primary))]  hover:cursor-pointer"
                                  >
                                    {player.personagem.atributos.vida_atual ===
                                      player.personagem.atributos.vida_maxima &&
                                      tipo === "Cerveja"
                                      ? "Vida cheia"
                                      : "Usar carta"}
                                  </Button>
                                )}
                                <DetalhesCarta carta={carta} />
                                {(turno === player.nome && player.nome === nome)
                                && (<Button
                                  className="bg-[hsl(var(--primary))] hover:cursor-pointer"
                                  onClick={() => descartarCarta(carta, indexPlayer, player.nome, tipo)}
                                >
                                  Descartar
                                </Button>)}
                              </CardFooter>
                            </Card>
                          );
                        })}
                      </div>
                          
                        )}
                        
                        
                      {turno === player.nome && (
                        <Button
                          onClick={async () => {
                            if (players[indexPlayer + 1]) {
                              await passarTurno(players[indexPlayer + 1].nome);
                              
                              const log: LogsCartas = {
                                nomeCarta: "Fim de Turno",
                                descricao: `${player.nome} passou a vez para ${players[indexPlayer + 1].nome}.`,
                                nomeJogador: `${player.nome}`
                              }
                              setLogs((prevLogs) => [...prevLogs, log]);
                              toast(
                                `${player.nome} passou a vez para ${players[indexPlayer + 1].nome}.`,
                              );

                              const novasCartas = await comprarCartas(players[indexPlayer + 1]);
                              players[indexPlayer + 1].cartas = players[indexPlayer + 1].cartas.concat(novasCartas);
                              const logCompra: LogsCartas = {
                                nomeCarta: "Compra",
                                descricao: `${players[indexPlayer + 1].nome} comprou ${novasCartas.length} cartas.`,
                                nomeJogador: `${player.nome}`
                              };
                              toast(
                                `${players[indexPlayer + 1].nome} comprou ${novasCartas.length} cartas.`,
                              );
                              setLogs((prevLogs) => [...prevLogs, logCompra]);
                            }
                            if (!players[indexPlayer + 1]) {
                              await passarTurno(players[0].nome);

                              const log: LogsCartas = {
                                nomeCarta: "Fim de Turno",
                                descricao: `${player.nome} passou a vez para ${players[0].nome}.`,
                                nomeJogador: `${player.nome}`
                              }
                              setLogs((prevLogs) => [...prevLogs, log]);
                              toast(
                                `${player.nome} passou a vez para ${players[0].nome}.`,
                              );
                              const novasCartas = await comprarCartas(players[0]);
                              players[0].cartas = players[0].cartas.concat(novasCartas);
                              const logCompra: LogsCartas = {
                                nomeCarta: "Compra",
                                descricao: `${players[0].nome} comprou ${novasCartas.length} cartas.`,
                                nomeJogador: `${player.nome}`
                              };
                              toast(
                                `${players[0].nome} comprou ${novasCartas.length} cartas.`,
                              );
                              setLogs((prevLogs) => [...prevLogs, logCompra]);                              
                            }
                          }}
                          className="bg-[hsl(var(--primary))] hover:cursor-pointer"
                          disabled={
                            player.personagem.atributos.vida_atual < player.cartas.length
                          }
                        >
                          {player.personagem.atributos.vida_atual < player.cartas.length ? (
                            `Descarte ou jogue ${player.cartas.length - player.personagem.atributos.vida_atual} cartas antes de passar o turno.`
                          ) : ("Passar o turno")}
                        </Button>
                      )}
                    </div>
                  </CardContent>
                </Card>
              ))}
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
  );
}

export default App;
