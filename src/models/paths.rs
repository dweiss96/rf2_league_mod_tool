use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Paths {
  pub modmgr: String,
  pub rf2: String,
  pub workshop: String,
}
