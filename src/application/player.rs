
use std::collections::LinkedList;
use application::gps_location::GpsLocation;

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
pub struct Player {
    name: String,
    icon_path: String,
    target: Option<Box<Player>>,
    targeting_players: LinkedList<Box<Player>>,
    location: GpsLocation,
    score: i32,
}

impl Player {
	pub fn new(name: String, icon_path: String) -> Player {
		let player = Player {
			name: name,
			icon_path: icon_path,
			target: None,
			targeting_players: LinkedList::new(),
			location: GpsLocation::new(),
			score: 0,
		};
		return player;
	}
	
	pub fn increment_score(&mut self) {
		self.score += 1;
	}
	
	pub fn get_score(&self) -> i32 {
		return self.score;
	}
	
	pub fn get_name(&self) -> &String {
		return &self.name;
	}
}
