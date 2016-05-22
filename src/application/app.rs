use std::io;

use application::command::Command;
use application::server::Server;

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
            		let id_result = input[1].trim().parse::<u64>();
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
            name: "RegisterPlayer".to_string(),
            description: "Registers a player with the server.  This can be done from any state.".to_string(),
            usage: "RegisterPlayer <PlayerName> <PathToPhoto>".to_string(),
            command_function: Box::new(|server: &mut Server, input: Vec<&str>| {
                server.register_player(input);
            }),
        });
    }
    
    fn run_command(&mut self, input: Vec<&str>) -> bool {
    	// Make sure the input has at least 1 entry
    	if input.len() == 0 {
    		return false;
    	}
    	
    	let command: &str = input[0].trim();
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

                    let split_input: Vec<&str> = input.split(' ').collect();
                    if !self.run_command(split_input) {
                    	println!("Failed to run the given command.");
                    }
                }
                _ => println!("Other."),
            };
        }
    }
}
