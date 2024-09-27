use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Role {
    Hitler,
    Fascist,
    Liberal,
}

impl Role {
    pub fn from(role_num: u8) -> Option<Self> {
        match role_num {
            0 => Some(Role::Hitler),
            1 => Some(Role::Fascist),
            2 => Some(Role::Liberal),
            _ => None,
        }
    }
    pub fn into(&self) -> u8 {
        match self {
            Role::Hitler => 0,
            Role::Fascist => 1,
            Role::Liberal => 2,
        }
    }
}
