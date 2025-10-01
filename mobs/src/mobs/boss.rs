use std::fmt::Debug;
#[derive(Debug, PartialEq, Clone)]
pub struct Boss {
    pub name: String,
    pub age: u32,
}

impl Boss {
    // an associated function
    pub fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}
