
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Sex {
	Female,
	Male,
	Other,
	Unspecified,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct User {
    pub user_name: String,
    pub icon_path: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: String,
    pub date_of_birth: i64,	
    pub sex: Sex,
}

impl User {
	pub fn new(name: &str) -> User {
		let user = User {
			user_name: name.to_string(),
			icon_path: "".to_string(),
			email: "".to_string(),
			first_name: "".to_string(),
			last_name: "".to_string(),
			middle_name: "".to_string(),
			date_of_birth: 0,	
			sex: Sex::Unspecified,
		};
		return user;
	}
}
