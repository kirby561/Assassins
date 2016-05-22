use std::collections::LinkedList;

use application::gps_location::GpsLocation;

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
}
