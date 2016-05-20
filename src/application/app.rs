use std::fmt;
use std::io;
use std::collections::LinkedList;

use application::command::Command;

#[derive(PartialEq)] // WTF?
pub enum AppState {
	Done,
	NotInGame,
	GameSetup,
	InGame,
}

pub struct App {
	app_state: AppState,
	commands: Vec<Command>,
}

impl App {
	pub fn new() -> App {
		 let mut app = App { app_state: AppState::NotInGame, commands: Vec::new() };
		 app.initialize_commands();
		 return app;
	}
	
	fn initialize_commands(&mut self) {
		self.commands.push(Command {
				name: "NewGame".to_string(),
				description: "Creates a new game and moves to the game setup state.".to_string(),
				usage: "NewGame".to_string(),
				command_function: Box::new(|this: &mut App,input: Vec<&str>| {
					this.start_new_game();
				})
			});
	}
	
	pub fn start_new_game(&mut self) {
		println!("The game has been started!");
	}
	
	pub fn register_player(&self, player_name: &str, player_photo: &str) {
		println!("Registering {} with photo {}", player_name, player_photo);
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
					match split_input[0].trim() {
						"NewGame" => self.start_new_game(),
						"RegisterPlayer" => self.register_player(split_input[1], split_input[2]),
						"DeletePlayer" => self.delete_player(split_input[1]),
						"ListPlayers" => self.list_players(),
						"Exit" => self.exit(),
						"Options" => self.print_options(),
						_ => println!("Invalid command.  Type Options for help.")
					};
				},
				AppState::GameSetup => println!("Game setup."),
				AppState::InGame => println!("In a game."),
				_ => println!("Other.")
			};
		}
	}
}