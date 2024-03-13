#[derive(Debug, Clone)]
pub struct Procedure {
    pub name: Box<str>,
    pub params: Vec<String>,
    pub code: String,
}

impl PartialEq for Procedure {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
