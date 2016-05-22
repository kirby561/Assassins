
#[derive(PartialEq)]
pub enum ServerInstanceState {
    Done,
    NotInGame,
    GameSetup,
    InGame,
}

pub struct ServerInstance {
    name: String,
    id: u64,
    state: ServerInstanceState,
}

impl ServerInstance {
	pub fn new(id: u64) -> ServerInstance {
		let mut server = ServerInstance {
			name: String::from("DefaultServerName"),
			id: id,
			state: ServerInstanceState::NotInGame,
		};
		return server;
	}
	
	pub fn destroy(&self) {
		println!("TODO: Do whatever we need to do to destroy this server instance.");
	}
	
    pub fn list_players(&self) {
        println!("Listing players...someday");
    }
    	
    pub fn start_new_game(&self) {
        println!("The game has been started!");
    }
    
    pub fn get_state_string(&self) -> String {
    	match self.state {
			ServerInstanceState::Done 		=> "Done".to_string(),
			ServerInstanceState::NotInGame 	=> "Not in a game".to_string(),
			ServerInstanceState::GameSetup  => "Game setup".to_string(),
			ServerInstanceState::InGame 	=> "In game".to_string(),
		}
    }
    
    pub fn get_id(&self) -> u64 {
    	return self.id;
    }
}
