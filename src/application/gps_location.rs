
pub struct GpsLocation {
    pub latitude: i64,
    pub longitude: i64,
}

impl GpsLocation {
	pub fn new() -> GpsLocation {
		let gpsLocation = GpsLocation {
			latitude: 0,
			longitude: 0,
		};
		return gpsLocation;
	}
}
