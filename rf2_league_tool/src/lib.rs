use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use rf2_league_creator::models::Config;
use rf2_league_creator::models::league::League;
use rf2_league_creator::models::paths::Paths;
use rf2_league_creator::models::car::Car;
use rf2_league_creator::models::driver::Driver;
use slint::{SharedString, VecModel};
use slint::Model;
use slint::ModelRc;
use std::rc::Rc;

pub mod controller {
    automod::dir!(pub "src/controller");
}
mod os_methods;
pub mod view;

slint::include_modules!();

pub type ThreadHandle = Arc<Mutex<Option<JoinHandle<()>>>>;

pub trait EmptyThreadHandleCreation {
    fn empty() -> ThreadHandle;
}

impl EmptyThreadHandleCreation for ThreadHandle {
    fn empty() -> ThreadHandle {
        Arc::new(Mutex::new(None))
    }
}

pub fn shared_string_to_option(shared: SharedString) -> Option<String> {
    if shared.is_empty() {
        None
    } else {
        Some(shared.to_string())
    }
}

impl Into<Config> for Configuration {
    fn into(self) -> Config {
        Config {
            paths: Paths {
                modmgr: self.modmgr_path.to_string(),
                rf2: "".to_string(),
                workshop: self.workshop_path.to_string(),
            },
            league: League {
                name: self.league_config.name.to_string(),
                car_category: self.league_config.category.to_string(),
                car_class: self.league_config.class.to_string(),
                upgrades_file_name: self.league_config.upgrade_file.to_string(),
                livery_file_prefix: shared_string_to_option(self.league_config.livery_file_prefix),
                livery_file_suffix: shared_string_to_option(self.league_config.livery_file_suffix),
                version_prefix: self.league_config.version_prefix.to_string(),
                cars: Vec::from_iter(self.league_config.cars
                    .map(|sc| Car {
                        id: sc.id.to_string(),
                        workshop_id: sc.workshop_id.to_string(),
                        version_overwrite: shared_string_to_option(sc.version_overwrite)
                    }).iter()),
                drivers: Vec::from_iter(self.league_config.driver
                .map(|sd| Driver {
                    name: sd.name.to_string(),
                    team: sd.team.to_string(),
                    car: sd.car.to_string(),
                    number: sd.number.try_into().unwrap_or(0u8),
                }).iter()),
            },
        }
    }
}

impl Into<Configuration> for Config {
    fn into(self) -> Configuration {
        Configuration {
            modmgr_path: SharedString::from(self.paths.modmgr),
            workshop_path: SharedString::from(self.paths.workshop),
            modmgr_found: true,
            workshop_found: true,

            league_config: LeagueConfiguration { 
                category: SharedString::from(self.league.car_category),
                class: SharedString::from(self.league.car_class),
                livery_file_prefix: SharedString::from(self.league.livery_file_prefix.unwrap_or_default()),
                livery_file_suffix: SharedString::from(self.league.livery_file_suffix.unwrap_or_default()),
                name: SharedString::from(self.league.name),
                upgrade_file: SharedString::from(self.league.upgrades_file_name),
                version_prefix: SharedString::from(self.league.version_prefix),
                cars: ModelRc::<CarConfiguration>::from(Rc::new(VecModel::from(Vec::from_iter(
                    self.league.cars.iter().map(|c| CarConfiguration {
                        id: SharedString::from(c.clone().id),
                        workshop_id: SharedString::from(c.clone().workshop_id),
                        version_overwrite: SharedString::from(c.clone().version_overwrite.unwrap_or_default()),
                    }).into_iter()
                )))),
                driver: ModelRc::from(Rc::from(VecModel::from(Vec::from_iter(
                    self.league.drivers.iter().map(|d| DriverConfiguration {
                        name: SharedString::from(d.clone().name),
                        car: SharedString::from(d.clone().car),
                        team: SharedString::from(d.clone().team),
                        number: d.number.into(),
                    }).into_iter()
                )))),
            }
        }
    }
}
