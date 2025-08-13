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
// NOTE: I'd like to use &str, but ran into lifetime issues so, strings it is.
async fn calculate_move(Json(payload): Json<Game>) -> (StatusCode, Json<Game>) {
    let mut board: Vec<String> = payload.board;
    let mut status: Status = Status::InProgress;

    if is_winner(&board, "x".to_string()) {
        // Human won :(
        info!("X wins, this shouldn't happen");
        status = Status::XWins;
    } else if is_winner(&board, "0".to_string()) {
        // Computer won :)
        info!("O wins!! Computers rule, humans drool");
        status = Status::OWins;
    } else if is_draw(&board, (0..9).collect()) {
        // A draw :|
        info!("It's a Draw");
        status = Status::Draw;
    } else if let Some(space) = can_win(&board, "o".to_string()) {
        // We can win! Take the square!
        make_move(&mut board, space);
        status = Status::OWins;
    } else if let Some(space) = best_move(&board) {
        // make some moves
        make_move(&mut board, space);
    }

    let new_turn = Game {
        board: board,
        status: status,
    };

    (StatusCode::OK, Json(new_turn))
}

fn can_win(board: &Vec<String>, letter: String) -> Option<usize> {
    let available_moves = possible_moves(&board, (0..9).collect());

    for space in available_moves {
        let mut test_board = board.clone();
        test_board[space] = letter.clone();
        if is_winner(&test_board, letter.clone()) {
            info!("{letter} will win with {space}");
            return Some(space);
        }
    }

    info!("Nope. No winning move for {letter}");
    None
}

fn best_move(board: &Vec<String>) -> Option<usize> {
    // If human can win on the next move, block them
    if let Some(space) = can_win(&board, "x".to_string()) {
        info!("Human can win, block them!");
        return Some(space);
    }

    // Opening moves
    if is_first_human_move(&board) {
        // If the human chooses the center square as their first move, pick a corner.
        if !is_space_free(&board, 4) {
            // Let's prioritize a corner space
            info!("The human chose the center square first, play a corner");
            return choose_corner(&board);
        }

        // The center is free, let's take it
        if is_space_free(&board, 4) {
            info!("play the center");
            return Some(4);
        }
    }

    // fork logic

    // Choose corner
    if let Some(rand_move) = choose_corner(board) {
        info!("Pick a corner {rand_move}");
        return Some(rand_move);
    }

    // As a last step choose a side
    let random_side_move: Option<usize> = choose_random_move(&board, vec![3, 7, 5, 1]);
    info!("Pick a side {random_side_move:#?}");

    random_side_move
}

fn is_first_human_move(board: &Vec<String>) -> bool {
    possible_moves(&board, (0..9).collect()).len() == 8
}

fn choose_corner(board: &Vec<String>) -> Option<usize> {
    if let Some(rand_move) = choose_random_move(&board, vec![0, 6, 2, 8]) {
        info!("Pick a corner");
        return Some(rand_move);
    }
    None
}

fn make_move(board: &mut Vec<String>, space: usize) {
    board[space] = "o".to_string();
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
