use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Person {
  pub name: String,
  pub age: u8,
  pub phones: Vec<String>,
}