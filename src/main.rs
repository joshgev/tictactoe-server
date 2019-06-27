use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use actix_cors::Cors;

pub mod board;

use board::{Board, Piece};

#[derive(Serialize)]
pub struct Message {
	message: String
}

#[derive(Deserialize, Serialize, Clone)]
struct MoveRequest {
	pub row: usize,
	pub column: usize
}


#[derive(Deserialize, Serialize, Clone)]
struct Player {
	pub name: String,
	pub piece: Piece
}

#[derive(Deserialize)]
struct NewGameRequest {
	player: Player
}

#[derive(Serialize)]
enum GameState {
	Ongoing,
	Draw,
	Won(Piece)
}

#[derive(Serialize)]
struct Game {
	pub id: Uuid,
	pub human_player: Player,
	pub computer_player: Player,
	pub state: GameState,
	board: Board
}

impl Game {
	pub fn new(human_player: Player, computer_player: Player) -> Self {
		Self {
			id: Uuid::new_v4(),
			human_player: human_player,
			computer_player: computer_player,
			board: Board::new(),
			state: GameState::Ongoing
		}
	}

	pub fn make_ai_move(&mut self, ai_player: &Player) -> Result<(), ()> {
		let mut empty_positions = self.board.get_empty_positions();
		if empty_positions.len() == 0 {
			return Err(())
		}

		thread_rng().shuffle(&mut empty_positions);
		let (x, y) = empty_positions[0];

		self.board.set(x, y, ai_player.piece.clone());
		Ok(())
	}

	pub fn update_game_state(&mut self) {
		if let Some(ref piece) = self.board.get_winning_piece() {
			self.state = GameState::Won(piece.clone());
			return;
		}

		if self.board.get_empty_positions().len() == 0 {
			self.state = GameState::Draw;
			return;
		}

		self.state = GameState::Ongoing
	}
}

struct APIState {
	games: HashMap<Uuid, Game>
}

impl APIState {
	pub fn new() -> Self {
		return Self {
			games: HashMap::new()
		}
	}
}


fn create(request: web::Json<NewGameRequest>, data: web::Data<Arc<Mutex<APIState>>>) -> impl Responder {
    // format!("Hello {}! id:{}", info.1, info.0)
    let human_player = request.player.clone();
    let computer_player = Player {
    	name: String::from("Computer"),
    	piece: match human_player.piece {
    		Piece::O => Piece::X,
    		Piece::X => Piece::O
    	}
    };
    	
    let make_first_move = match &computer_player.piece {
    	Piece::X => true,
    	Piece::O => false
    };

    let mut game = Game::new(human_player, computer_player.clone());
    if make_first_move {
    	let _ = game.make_ai_move(&computer_player);
    }
    
    let state = &mut *data.lock().unwrap();

    let response = HttpResponse::Ok().json(&game);
    state.games.insert(game.id.clone(), game);
    
    println!("n of games: {}", state.games.len());
    response
}

fn make_move(move_request: web::Json<MoveRequest>, data: web::Data<Arc<Mutex<APIState>>>, path: web::Path<Uuid>) -> HttpResponse {
	// let empty_positions = self.game.
	let state = &mut *data.lock().unwrap();
	// println!("n of games: {}", *games.len());
	if !state.games.contains_key(&path) {
		return HttpResponse::NotFound().json(Message{message: String::from("No such game.")})
	}

	let game = state.games.get_mut(&path).unwrap();

	let empty_positions = game.board.get_empty_positions();

	let row = move_request.row;
	let col = move_request.column;

	let element = empty_positions.into_iter().find(|&x| x == (row, col));
	if let None = element {
		return HttpResponse::BadRequest().json(Message{message: String::from("Not a valid spot")})
	}

	game.board.set(
		row,
		col,
		game.human_player.piece.clone()
	);

	game.update_game_state();

	match game.state {
		GameState::Won(_) => return HttpResponse::Ok().json(&game),
		GameState::Draw => return HttpResponse::Ok().json(&game),
		GameState::Ongoing => ()
	};
	
	let _ = game.make_ai_move(&game.computer_player.clone());

	game.update_game_state();

	HttpResponse::Ok().json(&game)
}

fn test() -> HttpResponse {
	let x: Piece = Piece::O;

	HttpResponse::Ok().json(x)
}

fn main() -> std::io::Result<()> {
	let data = Arc::new(Mutex::new(APIState::new()));
    HttpServer::new(
        move || App::new()
        	.data(data.clone())
        	.wrap(Cors::new().allowed_methods(vec!["GET", "POST"])) 
        	.service(web::resource("/games")
        		.route(web::post().to(create))
        		.route(web::get().to(test))
        	)
    		.service(web::resource("/games/{id}")
    			.route(web::post().to(make_move)))
    	)
        .bind("127.0.0.1:9000")?
        .run()
}
