
pub enum Sex {
	Female,
	Male,
	Other,
	Unspecified,
}

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
