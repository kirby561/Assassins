
use application::server_instance::ServerInstance;
use application::user::User;

#[derive(PartialEq)]
pub enum ServerState {
    Done,
    NotListening,
    Listening,
}

pub struct Server {
    name: String,
    state: ServerState,
    servers: Vec<ServerInstance>,
    clients: Vec<User>,
    next_id: u64,
}

impl Server {
	pub fn new() -> Server {
		let mut server = Server {
			name: String::from("DefaultServerName"),
			state: ServerState::NotListening,
			servers: Vec::new(),
			clients: Vec::new(),
			next_id: 0,
		};
		return server;
	}
	
	pub fn create_server_instance(&mut self) -> u64 {
		let id = self.get_next_id();
		self.servers.push(ServerInstance::new(id));
		return id;
	}
	
	pub fn destroy_server_instance(&mut self, id: u64) -> bool {
		let mut server_instance_index = 0;
		let mut found_server = false;
    	
    	// Find the server // ?? TODO: This should just be a hash table or something
	    for next_server_instance in (*self).servers.iter() {
	    	// Did we find it?
	    	if next_server_instance.get_id() == id {
	    		found_server = true;
	    		break;
	    	}
	    		
	    	// Nope, increment and continue
	    	server_instance_index += 1;
	    }
	    
	    // Remove it
	    if server_instance_index < self.servers.len() {
	    	self.servers[server_instance_index].destroy();
	    	self.servers.remove(server_instance_index);
	    }

		return found_server;
	}
    
    pub fn register_player(&self, input: Vec<&str>) {
    	println!("Register Player.");
    }
    
    pub fn get_state_string(&self) -> String {
    	match self.state {
			ServerState::Done 			=> "Done".to_string(),
			ServerState::Listening 		=> "Listening".to_string(),
			ServerState::NotListening 	=> "Not Listening".to_string(),
		}
    }
    
    fn get_next_id(&mut self) -> u64 {
    	self.next_id += 1;
    	return self.next_id;
    }
}
