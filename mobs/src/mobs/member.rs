use std::{fmt::Debug, panic};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Member {
    pub role: Role,
    pub age: u32,
}

impl Member {
    // a method
    pub fn get_promotion(&mut self) {
        self.role = match self.role {
            Role::Underboss => {
                panic!("Underbosses can't be promoted!");
            }
            Role::Caporegime => Role::Underboss,
            Role::Soldier => Role::Caporegime,
            Role::Associate => Role::Soldier,
        }
    }
}
