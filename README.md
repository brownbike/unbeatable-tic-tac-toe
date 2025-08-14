# unbeatable-tic-tac-toe

## Criteria

- Create a program that plays interactive Tic-Tac-Toe in the browser against a human player.
- The program must be unbeatable, it should always win or draw, never lose.
- The human player goes first.
- The program should:
  - Display the result at the end of the game
  - Automatically clear the board for a new round

## Architecture

- The game logic will be handled on a server, we'll use [axum](https://crates.io/crates/axum)
- The client will handle the UI in a React SPA using [Vite](https://vite.dev)

## Logic approach

- Game board is a 9 position array
- Create a list of the winning moves
- Check if either player won
- Check if it's a draw
- Check if computer can win on the next move
- Check if human can win on their next move, if so block
- Use [minimax](https://en.wikipedia.org/wiki/Minimax) algorithm to evaluate the best move

## Running the project

This project consists of two components, one that handles the game logic and the other that handles the UI. You need to run them both (instructions below), the easiest way is running them in dev mode. Follow the instruction and visit the default localhost (http://localhost:5173/) to play the game.

### Run the apps in dev mode

This assumes that you have [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) and [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm) installed. If you don't have `npm` I recommend using [nvm](https://github.com/nvm-sh/nvm) to handle your node versions. (bonus points, once installed you can just run `nvm use` in the root directory to install a known compatible version.)

```sh
# Server

# Switch to server dir
cd server

# Install dependencies and run server
cargo run

# or run with logging

 RUST_LOG=trace cargo run

# Default, this will spin up a server at http://localhost:3000/
```

```sh
# Client

# Switch to client dir
cd client

# Install dependencies
npm i

# Run client
npm run dev

# Default, this will spin up a server at http://localhost:5173/
```
