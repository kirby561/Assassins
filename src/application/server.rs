
pub struct Server {
    name: String,
    
}

impl Server {
	pub fn new() -> Server {
		let mut server = Server {
			name: String::from("DefaultServerName"),
		};
		return server;
	}
}
