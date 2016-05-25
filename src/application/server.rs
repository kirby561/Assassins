
use std::collections::HashMap;
use application::server_instance::ServerInstance;
use application::user::User;
use application::user_database::UserDatabase;

#[derive(PartialEq)]
pub enum ServerState {
    Done,
    NotListening,
    Listening,
}

pub struct Server {
    name: String,
    state: ServerState,
    servers: HashMap<u64, ServerInstance>,
    clients: Vec<User>,
    database: UserDatabase,
    next_id: u64,
}

impl Server {
	pub fn new() -> Server {
		let mut server = Server {
			name: String::from("DefaultServerName"),
			state: ServerState::NotListening,
			servers: HashMap::new(),
			clients: Vec::new(),
			database: UserDatabase::new(),
			next_id: 0,
		};
		return server;
	}
	
	pub fn create_server_instance(&mut self) -> u64 {
		let id = self.get_next_id();
		self.servers.insert(id, ServerInstance::new(id));
		return id;
	}
	
	pub fn destroy_server_instance(&mut self, id: u64) -> bool {
		match self.servers.remove(&id) {
			Some(_) => return true,
			None => return false,
		}
	}
   
    pub fn join_server_instance(&mut self, instance_id: u64, user_name: String) -> bool {
    	match self.servers.get_mut(&instance_id) {
    		Some(mut server_instance) => {
    			match self.database.get_user(user_name) {
		    		Some(user) => { 
		    			if server_instance.add_user(user) {
		    				return true;
		    			} else {
		    				return false;
		    			}
		    		},
		    		None => return false,
		    	}
    		},
    		None => return false,
    	}
    	
    	return true;
    }
    
    pub fn leave_server_instance(&mut self, instance_id: u64, user_name: String) -> bool {
    	match self.servers.get_mut(&instance_id) {
    		Some(mut server_instance) => {
    			match self.database.get_user(user_name) {
		    		Some(user) => { 
		    			if server_instance.remove_user(user.clone()) {
		    				return true;
		    			} else {
		    				return false;
		    			}
		    		},
		    		None => return false,
		    	}
    		},
    		None => return false,
    	}
    	
    	return true;
    }
    
    pub fn get_server_instance(&mut self, instance_id: u64) -> Option<&mut ServerInstance> {
    	return self.servers.get_mut(&instance_id);
    }
    
    pub fn register_user(&mut self, user: User) -> bool {
    	if self.database.user_exists(&user) {
    		return false;
    	}
    	self.database.register_user(user);
    	return true;
    }
    
    pub fn access_database(&self) -> &UserDatabase {
    	return &self.database;
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
