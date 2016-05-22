
#[derive(PartialEq)]
pub enum ServerState {
    Done,
    NotListening,
    Listening,
}

pub struct Server {
    name: String,
    state: ServerState,
}

impl Server {
	pub fn new() -> Server {
		let mut server = Server {
			name: String::from("DefaultServerName"),
			state: ServerState::NotListening,
		};
		return server;
	}
	
    pub fn start_new_game(&self, input: Vec<&str>) {
        println!("The game has been started!");
    }
    
    pub fn register_player(&self, input: Vec<&str>) {
    	println!("Register Player.");
    }

    pub fn list_players(&self) {
        println!("Listing players...someday");
    }
    
    pub fn get_state_string(&self) -> String {
    	match self.state {
			ServerState::Done 			=> "Done".to_string(),
			ServerState::Listening 		=> "Listening".to_string(),
			ServerState::NotListening 	=> "Not Listening".to_string(),
		}
    }
}
