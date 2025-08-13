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

    // TODO: break these out into functions.

    // TODO: see if we can win with a move

    // Check if human will win on the next move.
    if let Some(space) = can_block_move(&board) {
        make_move(&mut board, space);
    } else if is_space_free(&board, 4) {
        // The center is free, let's take it
        make_move(&mut board, 4);
    } else if let Some(rand_move) = choose_random_move(&board, vec![0, 6, 2, 8]) {
        // Let's prioritize a corner space
        make_move(&mut board, rand_move);
    } else if let Some(rand_move) = choose_random_move(&board, vec![3, 7, 5, 1]) {
        // Let's pick a random space
        make_move(&mut board, rand_move);
    }

    let new_turn = Game {
        board: board,
        status: Status::InProgress,
    };

    (StatusCode::OK, Json(new_turn))
}

fn can_block_move(board: &Vec<String>) -> Option<usize> {
    let available_moves = possible_moves(&board, (0..8).collect());
    for space in available_moves {
        let mut test_board = board.clone();
        test_board[space] = 'x'.to_string();
        if is_winner(&test_board, 'x'.to_string()) {
            info!("needs to be blocked!!!! {space}");
            return Some(space);
        }
    }

    info!("Nope. No block needed.");
    None
}

fn make_move(board: &mut Vec<String>, space: usize) {
    board[space] = 'o'.to_string();
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
