import { useState } from "react";
import "./App.css";

type Game = {
  board: string[];
  status: string;
};

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
  // TODO: maybe change this to status
  const [isNewGame, setIsNewGame] = useState<boolean>(true);
  const [board, setBoard] = useState<string[]>(new Array(9).fill(""));
  const [turn, setTurn] = useState<"x" | "o">("x");

  // NOTE: For simplicity sake, the human is always X and the computer is always O.
  async function handleHumanTurn(e: React.MouseEvent<HTMLDivElement>) {
    setIsNewGame(false);
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
        console.log("data: ", data.board);
        setBoard(data.board);
        setTurn("x");
      }
    }, 500);
  }

  async function handlePlayerAction(e: React.MouseEvent<HTMLDivElement>) {
    let newBoard = await handleHumanTurn(e);
    if (newBoard !== undefined) {
      handleComputerTurn(newBoard);
    } else {
      throw new Error("Board is undefined");
    }
  }

  const isHumanTurn = () => turn === "x";

  return (
    <>
      <div className="game">
        <h1 className="title">Unbeatable tic-tac-toe</h1>
        <div className="board">
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
        {/* 
          TODO: Add conditional instructions.   
          - Stating the game
          - Make you move
          - Computer is making a move (may need timeout)
          - Announce the winner
        */}
        <div>
          {isNewGame && (
            <>
              <p className="intro">
                Do you think you can beat a super smart computer at tic-tac-toe?
              </p>
              <p className="instructions">Click any square to begin.</p>
            </>
          )}
          {isHumanTurn() && <p className="instructions">It's X's turn</p>}
          {!isHumanTurn() && <p className="instructions">It's O's turn</p>}
        </div>
      </div>
    </>
  );
}

export default App;
