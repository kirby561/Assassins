extern crate time;

use application::player::Player;
use std::collections::LinkedList;
use self::time::Timespec;

// Defaults
const DEFAULT_GAME_TIME_LIMIT: i64 = 60 * 60 * 60 * 1000; // 60 minutes in ms

pub struct Game {
    time_limit: i64,
    start_time: Timespec,
    players: LinkedList<Player>,
}

impl Game {
	pub fn new() -> Game {
		let game = Game {
			time_limit: DEFAULT_GAME_TIME_LIMIT,
			start_time: Timespec::new(0, 0),
			players: LinkedList::new(),
		};
		return game;
	}
	
	pub fn clear_players(&mut self) {
		self.players.clear();
	}
	
	pub fn add_player(&mut self, player: Player) {
		self.players.push_back(player);
	}
	
	pub fn remove_player(&mut self, player: Player) {
		// ?? TODO
	}
	
	pub fn start(&mut self) {
		self.start_time = time::get_time();
	}
}
