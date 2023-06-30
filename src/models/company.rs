use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Company {
  pub name: String,
  pub workercount: u8,
  pub phones: Vec<String>,
}