import { useState, useEffect } from "react";
import "./App.css";

type Game = {
  board: string[];
  status: string;
};

// Enums are no longer recommended in TS, this is the recommended workaround.
// Personally I miss enums...
const GameStatus = {
  NewGame: "new-game",
  InProgress: "in-progress",
  XWins: "x-wins",
  OWins: "o-wins",
  Draw: "draw",
} as const;

type GameStatusType = (typeof GameStatus)[keyof typeof GameStatus];

// TODO: Break out into api file
async function calculateComputerMove(board: string[]) {
  let game = {
    board: board,
    status: "InProgress",
  };
  try {
    const requestOptions = {
      method: "PUT",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(game),
    };
    const resp = await fetch(
      "http://localhost:3000/calculate-move",
      requestOptions
    );
    let data = await resp.json();

    return data as Game;
  } catch (e) {
    console.error("There was an error calculating the next move: ", e);
  }
}

function App() {
  const [gameStatus, setGameStatus] = useState<GameStatusType>(
    GameStatus.NewGame
  );
  const [board, setBoard] = useState<string[]>(new Array(9).fill(""));
  const [turn, setTurn] = useState<"x" | "o">("x");

  useEffect(() => {
    // FIXME: put `hasWinner()` back

    if (gameStatus === GameStatus.Draw || gameStatus === GameStatus.OWins) {
      setTimeout(() => {
        setBoard(new Array(9).fill(""));
        setGameStatus(GameStatus.NewGame);
      }, 1500);
    }
  }, [gameStatus]);

  // NOTE: For simplicity sake, the human is always X and the computer is always O.
  async function handleHumanTurn(e: React.MouseEvent<HTMLDivElement>) {
    setGameStatus(GameStatus.InProgress);
    try {
      // Pull the `data-id` of of the click event
      const i = e.currentTarget.dataset.id;
      // Make sure our click event gives us a `data-id`
      if (i !== undefined) {
        const index = parseInt(i);
        // Make sure that `data-id` is a integer
        if (Number.isNaN(index)) {
          throw new Error("Square id is not a number");
        }

        // Copy the board
        let newBoard = [...board];
        // Update the board with the users move, the human is always x
        newBoard[index] = "x";
        // Update the board state
        setBoard(newBoard);
        setTurn("o");
        return newBoard;
      } else {
        throw new Error("Square id is undefined");
      }
    } catch (e) {
      console.error("There was an error selecting a square: ", e);
    }
  }

  async function handleComputerTurn(newBoard: string[]) {
    // Adding a setTimeout to make it a little less jarring.
    setTimeout(async () => {
      let data = await calculateComputerMove(newBoard);
      if (data !== undefined) {
        console.log("status: ", data.status);
        setBoard(data.board);
        setTurn("x");

        if (
          data.status === "XWins" ||
          data.status === "OWins" ||
          data.status === "Draw"
        ) {
          console.log("Game Over Man");
          setGameStatus(GameStatus[data.status]);
        }
      }
    }, 750);
  }

  async function handlePlayerAction(e: React.MouseEvent<HTMLDivElement>) {
    let newBoard = await handleHumanTurn(e);
    if (newBoard !== undefined) {
      handleComputerTurn(newBoard);
    } else {
      throw new Error("Board is undefined");
    }
  }

  const isHumanTurn = () =>
    turn === "x" &&
    (gameStatus === GameStatus.NewGame || gameStatus === GameStatus.InProgress);

  const isComputerTurn = () =>
    turn === "o" && gameStatus === GameStatus.InProgress;

  const hasWinner = () =>
    gameStatus === GameStatus.Draw ||
    gameStatus === GameStatus.XWins ||
    gameStatus === GameStatus.OWins;

  return (
    <>
      <div className="game">
        <h1 className="title">Unbeatable tic-tac-toe</h1>
        <div className={`board ${hasWinner() ? "has-winner" : ""}`}>
          {board.map((space, i) => {
            return (
              // TODO: break this out into a component
              <div
                key={i}
                className={`space ${space.toLowerCase()}`}
                onClick={handlePlayerAction}
                data-id={i}
              />
            );
          })}
        </div>
        <div>
          {gameStatus === GameStatus.NewGame && (
            <>
              <p className="intro">
                Do you think you can beat a super smart computer at tic-tac-toe?
              </p>
              <p className="instructions">Click any square to begin.</p>
            </>
          )}
          {isHumanTurn() && <p className="instructions">It's X's turn</p>}
          {isComputerTurn() && <p className="instructions">It's O's turn</p>}
          {gameStatus === GameStatus.Draw && (
            <p className="game-over">It's a Draw</p>
          )}
          {gameStatus === GameStatus.XWins && (
            <p className="game-over">X Wins!</p>
          )}
          {gameStatus === GameStatus.OWins && (
            <p className="game-over">O wins!</p>
          )}
        </div>
      </div>
    </>
  );
}

export default App;
