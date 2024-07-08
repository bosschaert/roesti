#[derive(PartialEq, Eq, Hash, Debug)]
pub struct TidalService {
    pub location: String
}

impl TidalService {
    pub fn location(&self) -> &str {
        &self.location
    }

    pub fn next_event(&self) -> u16 {
      42
    }
}