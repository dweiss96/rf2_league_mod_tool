use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Car {
  pub id: String,
  pub workshop_id: String,
  pub version_overwrite: String,
}
