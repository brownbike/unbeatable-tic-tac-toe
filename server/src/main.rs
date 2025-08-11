use axum::{
    routing::get,
    http::StatusCode,
    Json, Router,
};
use tower_http::cors::{CorsLayer, Any};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/calculate-move", get(calculate_move))
        .layer(cors);

    // run the app on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Running the server on http://localhost:3000/");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Welcome to tic-tac-toe"
}

// Json(payload): Json<Game>
// NOTE: For simplicity sake, the human is always X and the computer is always O.
async fn calculate_move() -> (StatusCode, Json<Game>) {
    let new_board = vec!["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "o".to_string()];

    let new_game = Game {
        board: new_board,
        status: Status::InProgress
    };

    (StatusCode::OK, Json(new_game))
}


#[derive(Serialize)]
struct Game {
    board: Vec<String>,
    status: Status,
}

#[derive(Serialize)]
enum Status {
    Draw, 
    XWins,
    OWins,
    InProgress
}