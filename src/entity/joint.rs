pub struct Joint {
    pub lat: f64,
    pub lon: f64,
}

impl Joint {
    pub fn new(lat: f64, lon: f64) -> Self {
        Joint { lat, lon }
    }
}
