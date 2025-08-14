use axum::{
    Json, Router,
    http::{Method, StatusCode},
    routing::{get, put},
};
use log::info;
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

// NOTE: Remember, the human is always X and the computer is always O.
async fn calculate_move(Json(payload): Json<Game>) -> (StatusCode, Json<Game>) {
    let mut board: Vec<String> = payload.board;
    let mut status: Status = Status::InProgress;

    if is_winner(&board, "x") {
        // Human won :(
        info!("X wins, this shouldn't happen");
        status = Status::XWins;
    } else if is_winner(&board, "o") {
        // Computer won :)
        info!("O wins!! Computers rule, humans drool");
        status = Status::OWins;
    } else if is_draw(&board) {
        // A draw :|
        info!("It's a Draw");
        status = Status::Draw;
    } else if let Some(winning_move) = can_win(&board, "o") {
        // We can win!
        make_move(&mut board, winning_move, "o");
        info!("O wins!! Computers rule, humans drool");
        status = Status::OWins;
    } else if let Some(best_move) = minimax_move(&board) {
        // Use some fancy math to calculate the best move
        make_move(&mut board, best_move, "o");
    }

    let new_turn = Game {
        board: board,
        status: status,
    };

    (StatusCode::OK, Json(new_turn))
}

fn can_win(board: &Vec<String>, letter: &str) -> Option<usize> {
    let available_moves = possible_moves(&board);

    for space in available_moves {
        let mut test_board = board.clone();
        make_move(&mut test_board, space, "o");
        if is_winner(&test_board, letter) {
            info!("{letter} will win with {space}");
            return Some(space);
        }
    }

    info!("Nope. No winning move for {letter}");
    None
}

// Evaluate all possible moves and find the best one
fn minimax(board: &Vec<String>, depth: i32, is_maximizing: bool) -> i32 {
    // Can we win?
    if is_winner(board, "o") {
        return 10 - depth;
    }

    // Do the humans win?
    if is_winner(board, "x") {
        return depth - 10;
    }

    // Is it a draw?
    if is_draw(board) {
        return 0;
    }

    let available_moves = possible_moves(board);

    if is_maximizing {
        // Score the computer moves
        let mut best_score = i32::MIN;
        for space in available_moves {
            let mut test_board = board.clone();
            make_move(&mut test_board, space, "o");
            let score = minimax(&test_board, depth + 1, false);
            best_score = best_score.max(score);
        }
        best_score
    } else {
        // Score the human moves
        let mut best_score = i32::MAX;
        for space in available_moves {
            let mut test_board = board.clone();
            make_move(&mut test_board, space, "x");
            let score = minimax(&test_board, depth + 1, true);
            best_score = best_score.min(score);
        }
        best_score
    }
}

fn minimax_move(board: &Vec<String>) -> Option<usize> {
    let available_moves = possible_moves(board);
    // Start with the smallest int possible
    let mut best_score = i32::MIN;
    let mut best_move = None;

    for space in available_moves {
        let mut test_board = board.clone();
        make_move(&mut test_board, space, "o");
        // Score every possible move
        let score = minimax(&test_board, 0, false);

        // We found a better move
        if score > best_score {
            best_score = score;
            best_move = Some(space);
        }
    }

    if let Some(mv) = best_move {
        info!("The best move {mv} scored {best_score}");
    }
    best_move
}

fn make_move(board: &mut Vec<String>, space: usize, letter: &str) {
    board[space] = letter.to_string();
}

fn is_winner(board: &Vec<String>, letter: &str) -> bool {
    let l = letter.to_string();

    return (board[0] == l && board[1] == l && board[2] == l) // across the top
    || (board[3] == l && board[4] == l && board[5] == l) // across the middle
    || (board[6] == l && board[7] == l && board[8] == l) // across the bottom
    || (board[0] == l && board[3] == l && board[6] == l) // down the left
    || (board[1] == l && board[4] == l && board[7] == l) // down the middle
    || (board[2] == l && board[5] == l && board[8] == l) // down the right
    || (board[0] == l && board[4] == l && board[8] == l) // diagonal
    || (board[6] == l && board[4] == l && board[2] == l); // diagonal
}

fn is_space_free(board: &Vec<String>, space: usize) -> bool {
    board[space].is_empty()
}

fn possible_moves(board: &Vec<String>) -> Vec<usize> {
    let mut possible_moves: Vec<usize> = vec![];
    let move_list: Vec<usize> = (0..9).collect();
    for i in move_list {
        if is_space_free(board, i) {
            possible_moves.push(i);
        }
    }
    possible_moves
}

fn is_draw(board: &Vec<String>) -> bool {
    possible_moves(board).len() == 0
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
