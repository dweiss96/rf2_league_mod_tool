use std::fs;
use std::path::Path;
use crate::models::league::League;
use crate::models::driver::Driver;
use crate::tasks::create_veh_file;

#[cfg(test)]
mod test {
    use crate::models::car::Car;
    use tempfile::tempdir;
    use super::*;
    fn get_league_cars_and_drivers() -> (League, Vec<Car>, Vec<Driver>) {
        let drivers = vec![Driver {
            name: String::from("Max Mustermann"),
            team: String::from("M&M Racing"),
            number: 42,
            car: String::from("CAR_GT3_2023")
        },Driver {
            name: String::from("Thomas Müller"),
            team: String::from("M&M Racing"),
            number: 69,
            car: String::from("CAR_GT3_2023")
        },Driver {
            name: String::from("John Stone"),
            team: String::from("Ipsum Racing"),
            number: 13,
            car: String::from("CAR_GTE_2023")
        },Driver {
            name: String::from("Jackie Meldrum"),
            team: String::from("Ipsum Racing"),
            number: 7,
            car: String::from("CAR_GTE_2023")
        }];

        let cars = vec![
            Car {
                id: "CAR_GT3_2023".to_string(),
                workshop_id: "123456789".to_string(),
                version_overwrite: Some("1.00".to_string())
            },
            Car {
                id: "CAR_GTE_2023".to_string(),
                workshop_id: "012345678".to_string(),
                version_overwrite: Some("1.00".to_string())
            }
        ];

        let league = League {
            name: String::from("Test Liga 23 S1"),
            car_class: String::from("TestLiga-23S1"),
            car_category: String::from("TestLiga, TestLiga 23 Saison 1"),
            upgrades_file_name: String::from("TestLigaUpgrades.ini"),
            livery_file_prefix: None,
            livery_file_suffix: Some(String::from("TST")),
            version_prefix: String::from("TST23"),
            cars: cars.clone(),
            drivers: drivers.clone()
        };

        return (league, cars, drivers)
    }

    fn work_in_tmp_dir<F>(func: F, subfolder: Option<String>) -> String where F: Fn(&str) -> () {
        // Create a directory inside of `std::env::temp_dir()`.
        let tmp_dir = tempdir().expect("Could not create test ");
        let tmp_dir_path = tmp_dir.path().to_str().unwrap();

        func(tmp_dir_path);

        let files = fs::read_dir(Path::new(tmp_dir_path).join(subfolder.unwrap_or_default())).unwrap()
            .map(|e| format!("{}", e.unwrap().file_name().to_str().unwrap()))
            .filter(|f| f.ne(".DS_Store"))
            .fold(String::new(), |acc,dir| if acc.is_empty() { format!("{}", dir) }else{ format!("{};{}", acc, dir)});

        // By closing the `TempDir` explicitly, we can check that it has
        // been deleted successfully. If we don't close it explicitly,
        // the directory will still be deleted when `dir` goes out
        // of scope, but we won't know whether deleting the directory
        // succeeded.
        tmp_dir.close().unwrap();
        return files;
    }

    #[test]
    fn copies_league_files() {
        // let (league, driver) = get_league_and_driver();
        let result_files = work_in_tmp_dir(|tmp_dir| {
            copy_league_files("tests/example_files", tmp_dir);
        }, None);
        assert_eq!("CAR_GTE_2023_v1.00.rfcmp;CAR_GT3_2023_v1.00.rfcmp",result_files)
    }

    #[test]
    fn copies_car_files() {
        let (_, cars, _) = get_league_cars_and_drivers();
        let car_id = cars.first().unwrap().id.as_str();
        let result_files = work_in_tmp_dir(|tmp_dir| {
            copy_car_files("tests/example_files", tmp_dir, car_id);
        }, Some(car_id.to_string()));
        assert_eq!("TestLigaUpgrades.ini;brand_logo.png",result_files)
    }

    #[test]
    fn copies_driver_files() {
        let (league, _, drivers) = get_league_cars_and_drivers();
        let result_files = work_in_tmp_dir(|tmp_dir| {
            copy_driver_files("tests/example_files", tmp_dir, league.clone(), drivers.first().unwrap().clone());
        }, Some(drivers.first().unwrap().clone().car));
        assert_eq!("42TST_region.dds;42TSTWindshieldIn.dds;42TST-icon-2048x1152.png;42TST.json;42TST-icon-512x288.png;42TSTicon.png;42TST-icon-1024x576.png;42TST-icon-128x72.png;42TST-icon-256x144.png;42TSTWindshieldOut.dds;42TST.dds",result_files)
    }
}

pub fn copy(cfg_dir: &str, tmp_dir: &str, league: League) {
    copy_league_files(cfg_dir, tmp_dir);
    for c in league.clone().cars {
        copy_car_files(cfg_dir, tmp_dir, c.id.as_str());
        // copy driver files for placeholder
        copy_driver_files(cfg_dir, tmp_dir, league.clone(), Driver {
            name: "Guest Driver".to_string(),
            number: 0,
            team: league.clone().name,
            car: c.id,
        });
    }
    for d in league.clone().drivers {
        copy_driver_files(cfg_dir, tmp_dir, league.clone(), d);
    }
}

fn copy_league_files(cfg_dir: &str, tmp_dir: &str) {
    let src_dir = Path::new(cfg_dir).join("_league");
    let out_dir = Path::new(tmp_dir);

    fs::read_dir(src_dir.clone()).unwrap().for_each(|entry| {
        let de = entry.unwrap();
        let _ = fs::copy(src_dir.join(de.file_name()), out_dir.join(de.file_name()));
    });
}

fn copy_car_files(cfg_dir: &str, tmp_dir: &str, car_dir: &str) {
    let src_dir = Path::new(cfg_dir).join(car_dir);
    let out_dir = Path::new(tmp_dir).join(car_dir);

    fs::create_dir_all(out_dir.to_str().unwrap()).unwrap();
    fs::read_dir(src_dir.clone()).unwrap().for_each(|entry| {
        let efn = entry.unwrap().file_name();
        let entry_name = efn.to_str().unwrap();
        // match entry.unwrap().file_name().to_str().unwrap() {
        match entry_name {
            "skins" => {},
            "_vehicle.veh" => {},
            ".DS_Store" => {},
            file => {
                let _ = fs::copy(src_dir.join(file), out_dir.join(file));
            }
        }
    });
}

fn copy_driver_files(cfg_dir: &str, tmp_dir: &str, league: League, driver: Driver) {
    let veh_file: String = String::from_utf8_lossy(
        &fs::read(Path::new(cfg_dir).join(driver.clone().car).join("_vehicle.veh")).unwrap()
    ).to_string();
    let src_dir = Path::new(cfg_dir).join(driver.clone().car).join("skins").join(format!("{}", driver.clone().number));
    let out_dir = Path::new(tmp_dir).join(driver.clone().car);

    let skin_file_name = format!("{}{}{}", league.livery_file_prefix.clone().unwrap_or_default(), driver.number, league.livery_file_suffix.clone().unwrap_or_default());

    let updated_veh = create_veh_file::substitute_veh_fields(veh_file.as_str(), league, driver);
    let _ = fs::write(out_dir.join(format!("{}.veh",skin_file_name.clone())), updated_veh);

    fs::create_dir_all(out_dir.to_str().unwrap()).unwrap();
    fs::read_dir(src_dir.clone()).unwrap().for_each(|entry| {
        let fname = entry.unwrap().file_name();
        let new_fname = fname.clone().to_str().unwrap().replace("skin", skin_file_name.as_str());

        let _ = fs::copy(src_dir.join(fname), out_dir.join(new_fname));
    });
}