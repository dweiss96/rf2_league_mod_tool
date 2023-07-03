automod::dir!(pub "src/models");

use crate::models::league::League;
use crate::models::paths::Paths;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub paths: Paths,
    pub league: League,
}
