
#[derive(Hash, Clone, Eq, PartialEq, Debug)]
pub struct GpsLocation {
    pub latitude: i64,
    pub longitude: i64,
}

impl GpsLocation {
	pub fn new() -> GpsLocation {
		let gps_location = GpsLocation {
			latitude: 0,
			longitude: 0,
		};
		return gps_location;
	}
}
