use serde::{Deserialize, Serialize};

use crate::models::car::Car;
use crate::models::driver::Driver;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct League {
  pub name: String,
  pub car_category: String,
  pub car_class: String,
  pub upgrades_file_name: String,
  pub livery_file_prefix: Option<String>,
  pub livery_file_suffix: Option<String>,
  pub version_prefix: String,
  pub cars: Vec<Car>,
  pub drivers: Vec<Driver>,
}
