import type { GameStatusType } from "../App";

type Game = {
  board: ("x" | "o")[];
  status: GameStatusType;
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

export default calculateComputerMove;
