
use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use application::command::Command;
use application::server::Server;
use application::user::User;

#[derive(PartialEq)]
pub enum AppState {
    Running,
    Done,
}

pub struct App {
    app_state: AppState,
    server: Server,
    commands: Vec<Command>,
}

impl App {
    pub fn new() -> App {
        let mut app = App {
            app_state: AppState::Done,
            server: Server::new(),
            commands: Vec::new(),
        };
        app.initialize_commands();
        return app;
    }

    fn initialize_commands(&mut self) {
        self.commands.push(Command {
            name: "CreateServer".to_string(),
            description: "Creates a new server instance for players to join.".to_string(),
            usage: "CreateServer".to_string(),
            command_function: Box::new(|server: &mut Server, input: Vec<&str>| {
                let id = server.create_server_instance();
                println!("Created server with id {}", id);
            }),
        });
        
        self.commands.push(Command {
            name: "DestroyServer".to_string(),
            description: "Destroys the given server.".to_string(),
            usage: "DestroyServer <ServerInstanceId>".to_string(),
            command_function: Box::new(|server: &mut Server, input: Vec<&str>| {
            	if input.len() != 2 {
            		println!("Invalid arguments.  See usage.");
            	} else {
            		let id_result = input[1].parse::<u64>();
            		match id_result {
            			Ok(id) => {
            				if server.destroy_server_instance(id) {
				                println!("Destroyed server with id {}", id);
            				} else {
            					println!("No server found with id {}", id);
            				}
            			}
            			Err(error) => {
	            			println!("Invalid server id {}", input[1]);
            			}
            		}
            	}
            }),
        });
        
        self.commands.push(Command {
            name: "JoinServer".to_string(),
            description: "Joins the specified server with the specified user.".to_string(),
            usage: "JoinServer <ServerInstanceId> <UserName>".to_string(),
            command_function: Box::new(|server: &mut Server, input: Vec<&str>| {
            	if input.len() != 3 {
            		println!("Invalid arguments.  See usage.");
            	} else {
            		let id_result = input[1].parse::<u64>();
            		match id_result {
            			Ok(id) => {
							// Join the server
							if server.join_server_instance(id, input[2].to_string()) {
							    println!("{} has joined server instance {}", input[2], id);
		            		} else {
		            			println!("No server found with id {} or {} is already in that server.", id, input[2]);
		            		}
            			}
            			Err(error) => {
	            			println!("Invalid server id {}", input[1]);
            			}
            		}
            	}
            }),
        });
        
        self.commands.push(Command {
            name: "LeaveServer".to_string(),
            description: "Leaves the specified server for the specified user.".to_string(),
            usage: "LeaveServer <ServerInstanceId> <UserName>".to_string(),
            command_function: Box::new(|server: &mut Server, input: Vec<&str>| {
            	if input.len() != 3 {
            		println!("Invalid arguments.  See usage.");
            	} else {
            		let id_result = input[1].parse::<u64>();
            		match id_result {
            			Ok(id) => {
							// Join the server
							if server.leave_server_instance(id, input[2].to_string()) {
							    println!("{} has left server instance {}", input[2], id);
		            		} else {
		            			println!("No server found with id {} or {} is not in that server.", id, input[2]);
		            		}
            			}
            			Err(error) => {
	            			println!("Invalid server id {}", input[1]);
            			}
            		}
            	}
            }),
        });
        
        self.commands.push(Command {
            name: "RegisterUser".to_string(),
            description: "Registers a user with the server.  This can be done from any state.".to_string(),
            usage: "RegisterUser <PlayerName> <PathToPhoto?>".to_string(),
            command_function: Box::new(|server: &mut Server, input: Vec<&str>| {
            	if input.len() > 1 {
            		let user_name = input[1];
            		
            		// Name is required first input
		           	let mut user = User::new(user_name);
		           	
		           	// Photo?
		           	if input.len() > 2 {
		           		user.icon_path = input[2].to_string();
		           	}
		           	
		           	// ?? TODO: Add other fields.
		           	
		            if server.register_user(user) {
		            	println!("Registered {}", user_name);
		            } else {
		            	println!("{} was already registered.", user_name);
		            }
            	} else {
            		println!("Invalid input for RegisterUser.  See usage.");
            	}
            }),
        });
        
        self.commands.push(Command {
            name: "ListUsers".to_string(),
            description: "Lists all the users currently in the database.".to_string(),
            usage: "ListUsers".to_string(),
            command_function: Box::new(|server: &mut Server, input: Vec<&str>| {
            	server.access_database().list_users();
            }),
        });        
    }
    
    fn execute_command(&mut self, input: Vec<&str>) -> bool {
    	// Make sure the input has at least 1 entry
    	if input.len() == 0 {
    		return false;
    	}
    	
    	let command: &str = input[0];
    	let mut command_index = 0;
    	
    	// Find the command // ?? TODO: This should just be a hash table.
	    for next_command in (*self).commands.iter() {
	    	// Did we find it?
	    	if next_command.name == command {
	    		break;
	    	}
	    		
	    	// Nope, increment and continue
	    	command_index += 1;
	    }
    	
    	if command_index < (*self).commands.len() {	
			(*(*self).commands[command_index].command_function)(&mut self.server, input);
	    	return true;
    	}
    	
    	// Not found
    	return false;
    }
    
    pub fn execute_script(&mut self, script: &str) -> bool {
    	let f = match File::open(script) {
		    Ok(file) => {
		    	let mut result = true;
		    	let mut file_reader = BufReader::new(&file);
			    for line in file_reader.lines() {
			        let l = line.unwrap();
                    result &= self.handle_input(&l);
			    }
			    return result;
		    },
		    Err(e) => {
		        println!("{}", e);
		        return false;
		    }
		};
    	return false;
    }
    
    pub fn handle_input(&mut self, input: &String) -> bool {
    	let split_input: Vec<&str> = input.trim().split(' ').collect();
        
        // Special case for scripts
        if split_input.len() > 0 && split_input[0] == "RunScript".to_string() {
        	if split_input.len() != 2 {
        		println!("Please provide a script file.");
        		return false;
        	} else {
        		if !self.execute_script(split_input[1]) {
        			println!("Failed to run the script at {}", split_input[1]);
        			return false;
        		}
        	}
        } else if split_input.len() > 0 && split_input[0].trim() != "".to_string() {
        	// Run the command 
	        if !self.execute_command(split_input) {
	        	println!("Failed to run the given command.");
	        	return false;
	        }
        }
        return true;
    }

    pub fn exit(&mut self) {
        println!("Exiting the app.");
        self.app_state = AppState::Done;
    }

    pub fn print_options(&self) {
        println!("Printing the options for the current state.");
    }

    pub fn run(&mut self) {
    	self.app_state = AppState::Running;
    	
        while self.app_state != AppState::Done {
            match self.app_state {
                AppState::Running => {
                    // Print the server's state
                    println!("<{}>", self.server.get_state_string());
                    
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();

                    self.handle_input(&input);
                }
                _ => println!("Other."),
            };
        }
    }
}
