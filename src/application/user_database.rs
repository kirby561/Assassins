
use application::user::User;
use std::collections::HashMap;

pub struct UserDatabase {
	// Temporary until we have a real database
	users: HashMap<String, User>,
}

impl UserDatabase {
	pub fn new() -> UserDatabase {
		let userDatabase = UserDatabase {
			users: HashMap::new(),
		};
		return userDatabase;
	}
	
	pub fn user_exists(&self, user: &User) -> bool {
		return self.users.contains_key(&user.user_name);
	}
	
	pub fn register_user(&mut self, user: User) {
		self.users.insert(user.user_name.to_string(), user);
	}
	
	pub fn get_user(&self, user_name: String) -> Option<&User> {
		return self.users.get(&user_name);
	}
	
	pub fn list_users(&self) {
		println!("Users:");
		for (key, user) in &self.users {
		    println!("\t{:?}", user);
		}
	}
}
