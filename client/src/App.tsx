import { useState } from "react";
import "./App.css";

// TODO: create enum for x|o

function App() {
  const [isNewGame, setIsNewGame] = useState<boolean>(true);
  const [board, setBoard] = useState<string[]>(new Array(9).fill(""));

  function handlePlaySquare(e: React.MouseEvent<HTMLDivElement>) {
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
        // Update the board with the users move
        newBoard[index] = "x";
        // Update the board state
        setBoard(newBoard);
      } else {
        throw new Error("Square id is undefined");
      }
    } catch (e) {
      console.error("There was an error selecting a square: ", e);
    }
  }

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
                onClick={handlePlaySquare}
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
        {isNewGame && (
          <>
            <p className="description">
              Do you think you can beat a super smart computer at tic-tac-toe?
            </p>
            <p className="instructions">Click any square to begin.</p>
          </>
        )}
      </div>
    </>
  );
}

export default App;
