use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Person {
    pub first_name: String,
    pub last_name: Option<String>,
    pub pet_name: Option<String>,
    pub doctor_name: Option<String>,
}