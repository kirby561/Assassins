
extern crate time;

use std::collections::HashMap;
use application::player::Player;
use self::time::Timespec;

// Defaults
const DEFAULT_GAME_TIME_LIMIT: i64 = 60 * 60 * 60 * 1000; // 60 minutes in ms

pub struct Game {
    time_limit: i64,
    start_time: Timespec,
    players: HashMap<String, Player>,	
}

impl Game {
	pub fn new() -> Game {
		let game = Game {
			time_limit: DEFAULT_GAME_TIME_LIMIT,
			start_time: Timespec::new(0, 0),
			players: HashMap::new(),
		};
		return game;
	}
	
	pub fn clear_players(&mut self) {
		self.players.clear();
	}
	
	pub fn add_player(&mut self, player: Player) {
		self.players.insert(player.get_name().clone(), player.clone());
	}
	
	pub fn get_players(&mut self) -> &HashMap<String, Player> {
		return &self.players;
	}
	
	pub fn get_players_mut(&mut self) -> &mut HashMap<String, Player> {
		return &mut self.players;
	}
	
	pub fn remove_player(&mut self, player: Player) {
		self.players.remove(player.get_name());
	}
	
	pub fn start(&mut self) {
		self.start_time = time::get_time();
	}
}
