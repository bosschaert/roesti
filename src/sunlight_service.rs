pub struct SunlightService {
    pub location: String
}

impl SunlightService {
    pub fn location(&self) -> &str {
        &self.location
    }

    pub fn next_sundown(&self) -> u16 {
        22
    }
  
    pub fn next_sunrise(&self) -> u16 {
        8
    }
}
