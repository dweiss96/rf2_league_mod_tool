use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Driver {
  pub name: String,
  pub number: u8,
  pub car: String,
  pub team: String,
}
