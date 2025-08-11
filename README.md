# unbeatable-tic-tac-toe

## Criteria

- Create a program that plays interactive Tic-Tac-Toe in the browser against a human player.
- Your program must be unbeatable—it should always win or draw, never lose.
- The human player goes first.
- The program should:
  - Display the result at the end of the game
  - Automatically clear the board for a new round

## Architecture

- The game logic will be handled on the server using [axum](https://crates.io/crates/axum)
- The client will handle the UI in a React SPA using [Vite](https://vite.dev)

## Logic approach

- Create a list of the winning moves
- Have the computer choose the center square first (if available)
- Have the computer prioritize the corners
- Check if human can win on their next move, if so block
- Have computer check if they can win on the next move
- Game board is a 9 position array
- Server handles the logic
- Client handles the UI

## Running the project

TODO: It's probably best to compile down to a server binary and run the client prod build

### Run the apps in dev mode

```
# Install dependencies
cd client && npm i
cd server && cargo build

# Run client
cd client
npm run dev

# Run server
cd server
cargo run
```
