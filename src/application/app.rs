use std::fmt;
use std::io;
use std::collections::LinkedList;

use application::command::Command;
use application::server::Server;

#[derive(PartialEq)] // WTF?
pub enum AppState {
    Done,
    NotInGame,
    GameSetup,
    InGame,
}

pub struct App {
    app_state: AppState,
    server: Server,
    commands: Vec<Command>,
}

impl App {
    pub fn new() -> App {
        let mut app = App {
            app_state: AppState::NotInGame,
            server: Server::new(),
            commands: Vec::new(),
        };
        app.initialize_commands();
        return app;
    }

    fn initialize_commands(&mut self) {
        self.commands.push(Command {
            name: "NewGame".to_string(),
            description: "Creates a new game and moves to the game setup state.".to_string(),
            usage: "NewGame".to_string(),
            command_function: Box::new(|input: Vec<&str>| {
                println!("Running {}", input[0]);
            }),
        });
        
        self.commands.push(Command {
            name: "RegisterPlayer".to_string(),
            description: "Registers a player with the server.  This can be done from any state.".to_string(),
            usage: "RegisterPlayer <PlayerName> <PathToPhoto>".to_string(),
            command_function: Box::new(|input: Vec<&str>| {
                println!("Running {}", input[0]);
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
    	
    	// Find the command
	    for next_command in (*self).commands.iter() {
	    	// Did we find it?
	    	if next_command.name == command {
	    		break;
	    	}
	    		
	    	// Nope, increment and continue
	    	command_index += 1;
	    }
    	
    	
    	if command_index < (*self).commands.len() {	
	    	println!("Next command: {}", command_index);
			(*(*self).commands[command_index].command_function)(input);
	    	return true;
    	}
    	
    	// Not found
    	return false;
    }

    pub fn start_new_game(&mut self, input: Vec<&str>) {
        println!("The game has been started!");
    }

    pub fn register_player(&self, input: Vec<&str>) {
        println!("Registering");
    }

    pub fn delete_player(&self, player_name: &str) {
        println!("Deleting player {}", player_name);
    }

    pub fn list_players(&self) {
        println!("Listing players...someday");
    }

    pub fn exit(&mut self) {
        println!("Exiting the app.");
        self.app_state = AppState::Done;
    }

    pub fn print_options(&self) {
        println!("Printing the options for the current state.");
    }

    pub fn run(&mut self) {
        while self.app_state != AppState::Done {
            match self.app_state {
                AppState::NotInGame => {
                    println!("<Not in a game.>");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();

                    let split_input: Vec<&str> = input.split(' ').collect();
                    if !self.run_command(split_input) {
                    	println!("Failed to run the given command.");
                    }
                }
                AppState::GameSetup => println!("Game setup."),
                AppState::InGame => println!("In a game."),
                _ => println!("Other."),
            };
        }
    }
}
