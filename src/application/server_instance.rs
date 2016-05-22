
use application::user::User;
use application::game::Game;
use application::player::Player;

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
    connected_users: Vec<User>,	
    game: Game,
}

impl ServerInstance {
	pub fn new(id: u64) -> ServerInstance {
		let server = ServerInstance {
			name: String::from("DefaultServerName"),
			id: id,
			state: ServerInstanceState::GameSetup,
			connected_users: Vec::new(),
			game: Game::new(),
		};
		return server;
	}
	
	pub fn destroy(&self) {
		println!("TODO: Do whatever we need to do to destroy this server instance.");
	}
	
    pub fn list_users(&self) {
        println!("Listing users...someday");
    }
    
    pub fn start_game(&mut self) {
        println!("The game has been started!");
        self.state = ServerInstanceState::InGame;
        
        // Grab whatever users are connected and start the game
        self.game.clear_players();
     
	    for user in self.connected_users.iter() {
	    	let new_player = Player::new(user.user_name.clone(), user.icon_path.clone());
	    	self.game.add_player(new_player);
	    }
	    
	    self.game.start();
    }
    
    pub fn add_user(&mut self, user: User) {
    	self.connected_users.push(user);
    }
    
    pub fn remove_user(&mut self, user: User) {
    	// ?? TODO
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
