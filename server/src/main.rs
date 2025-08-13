use axum::{
    Json, Router,
    http::{Method, StatusCode},
    routing::{get, put},
};
use log::info;
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("tower_http=trace"))
                .unwrap(),
        )
        .init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::PUT])
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/calculate-move", put(calculate_move))
        .layer(cors);

    // run the app on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    info!("Running the server on http://localhost:3000/");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Welcome to tic-tac-toe"
}

// NOTE: For simplicity sake, the human is always X and the computer is always O.
async fn calculate_move(Json(payload): Json<Game>) -> (StatusCode, Json<Game>) {
    let mut board: Vec<String> = payload.board;

    // Check for human winner
    if is_winner(&board, "x".to_string()) {
        info!("X wins, this shouldn't happen");
        let new_turn = Game {
            board: board,
            status: Status::XWins,
        };

        return (StatusCode::OK, Json(new_turn));
    }

    // Check for computer winner
    if is_winner(&board, "0".to_string()) {
        info!("O wins!! Computers rule, humans drool");
        let new_turn = Game {
            board: board,
            status: Status::OWins,
        };

        return (StatusCode::OK, Json(new_turn));
    }

    // Check for a draw
    if is_draw(&board, (0..8).collect()) {
        info!("It's a Draw");
        let new_turn = Game {
            board: board,
            status: Status::Draw,
        };

        return (StatusCode::OK, Json(new_turn));
    }

    // TODO: Check if human will win on the next move.
    // loop through the open squares and see if they win, then block them.

    // TODO: break these out into functions.
    // Try to take the center square
    if board[4].is_empty() {
        board[4] = 'o'.to_string();
    } else {
        // Try to take a random corner
        let random_move = choose_random_move(&board, vec![0, 6, 2, 8]);
        if let Some(rand_move) = random_move {
            board[rand_move] = 'o'.to_string();
        } else if random_move.is_none() {
            // Try to take a random square
            let random_move = choose_random_move(&board, vec![3, 7, 5, 1]);
            if let Some(rand_move) = random_move {
                board[rand_move] = 'o'.to_string();
            }
        }
    }

    let new_turn = Game {
        board: board,
        status: Status::InProgress,
    };

    (StatusCode::OK, Json(new_turn))
}

fn is_winner(board: &Vec<String>, letter: String) -> bool {
    (board[0] == letter && board[1] == letter && board[2] == letter) // across the top
    || (board[3] == letter && board[4] == letter && board[5] == letter) // across the middle
    || (board[6] == letter && board[7] == letter && board[8] == letter) // across the bottom
    || (board[0] == letter && board[3] == letter && board[6] == letter) // down the left
    || (board[1] == letter && board[4] == letter && board[7] == letter) // down the middle
    || (board[2] == letter && board[5] == letter && board[8] == letter) // down the right
    || (board[0] == letter && board[4] == letter && board[8] == letter) // diagonal
    || (board[6] == letter && board[4] == letter && board[2] == letter) // diagonal
}

fn is_space_free(board: &Vec<String>, space: usize) -> bool {
    board[space].is_empty()
}

fn possible_moves(board: &Vec<String>, moves_list: Vec<usize>) -> Vec<usize> {
    let mut possible_moves: Vec<usize> = vec![];
    for i in moves_list {
        if is_space_free(board, i) {
            possible_moves.push(i);
        }
    }
    possible_moves
}

fn is_draw(board: &Vec<String>, moves_list: Vec<usize>) -> bool {
    possible_moves(board, moves_list).len() == 0
}

fn choose_random_move(board: &Vec<String>, moves_list: Vec<usize>) -> Option<usize> {
    let possible_moves = possible_moves(board, moves_list);

    if possible_moves.len() != 0 {
        let random_move = possible_moves.choose(&mut rand::rng());
        info!("{random_move:?}");
        return random_move.copied();
    } else {
        return None;
    }
}

// fn make_move(mut board: Vec<String>, space: usize) {
//     board[space] = 'o'.to_string();
// }

#[derive(Serialize, Deserialize)]
struct Game {
    board: Vec<String>,
    status: Status,
}

#[derive(Serialize, Deserialize)]
enum Status {
    Draw,
    XWins,
    OWins,
    InProgress,
}
