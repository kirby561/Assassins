use application::App;

pub struct Command {
	pub name: String,
	pub usage: String,
	pub description: String,
	pub command_function: Box<Fn(&mut App,Vec<&str>)>,
}
