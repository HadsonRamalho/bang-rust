import { useState } from 'react'
import './App.css'
import { startGame } from './services/game/game';
import { Jogador } from './interfaces/player/player';
import { Jogo } from './interfaces/game/game';

function App() {
  
  const [players, setPlayers] = useState<Jogador[]>([]);

  const [qtdPlayers, setQtdPlayers] = useState(0);

  const [playerNames, setPlayerNames] = useState<string[]>(Array(qtdPlayers).fill(''));

  const loadGame = async () => {
    const res: Jogo = await startGame(playerNames);
    console.log(res);
    setPlayers(res.jogadores);
  }

  return (
    <>
      <div className="card">
        <div>
        <label htmlFor="qtdplayers">Quantidade de jogadores</label>
        <input id="qtdplayers"
        max={7}
        min={4}
        onChange={(e) => setQtdPlayers(parseInt(e.target.value))}
        />
        </div>

        {(qtdPlayers > 3 && qtdPlayers < 8) && (
          <div>
            {Array.from({length: qtdPlayers}).map((_, index) => (
              <div key={index}>
                <label htmlFor={`player${index}`}>Nome do jogador {index + 1}</label>
                <input id={`player${index}`}
                onChange={(e) => {
                  const newPlayerNames = [...playerNames];
                  newPlayerNames[index] = e.target.value;
                  setPlayerNames(newPlayerNames);
                }}/>
              </div>
            ))}
            </div>
        )}

        <button onClick={async () => {await loadGame()}}>
          Definir funções
        </button>
        <div>
          Jogadores: 
          {players.map((player, index) => (
            <div key={index}>
              Nome: {player.nome} - Função: {player.funcao.nome}
            </div>
          ))}
        </div>
      </div>
    </>
  )
}

export default App
