use std::ffi::OsString;
use std::fs;
use std::path::Path;
use crate::models::league::League;
use crate::models::driver::Driver;
use crate::tasks::create_veh_file;

pub fn copy(cfg_dir: &str, league: League, driver: Driver) {
    copy_league_files(cfg_dir);
    league.cars.iter().for_each(|car| {
        copy_car_files(cfg_dir, car.id.as_str());
        copy_driver_files(cfg_dir, car.id.as_str(), league, driver)
    })
}

fn copy_league_files(cfg_dir: &str) {
    let src_dir = Path::new(cfg_dir).join("_league");
    let out_dir = Path::new(cfg_dir).join(".out");

    fs::read_dir(src_dir.clone()).unwrap().for_each(|entry| {
        let de = entry.unwrap();
        fs::copy(src_dir.join(de.file_name()), out_dir.join(de.file_name()));
    });
}

fn copy_car_files(cfg_dir: &str, car_dir: &str) {
    let src_dir = Path::new(cfg_dir).join(car_dir);
    let out_dir = Path::new(cfg_dir).join(".out").join(car_dir);

    fs::read_dir(src_dir.clone()).unwrap().for_each(|entry| {
        match entry.unwrap().file_name().to_str().unwrap() {
            "skins" => {},
            "_vehicle.veh" => {},
            file => {fs::copy(src_dir.join(file), out_dir.join(file));}
        }
    });
}

fn copy_driver_files(cfg_dir: &str, car_dir: &str, league: League, driver: Driver) {
    let veh_file: String = String::from_utf8_lossy(
        &fs::read(Path::new(cfg_dir).join(car_dir).join("_vehicle.veh")).unwrap()
    ).to_string();
    let src_dir = Path::new(cfg_dir).join(car_dir).join("skins");
    let out_dir = Path::new(cfg_dir).join(".out").join(car_dir);

    let skin_file_name = format!("{}{}{}", league.livery_file_prefix, driver.number, league.livery_file_suffix);

    let updated_veh = create_veh_file::substitute_veh_fields(veh_file.as_str(), league, driver);
    fs::write(out_dir.join(skin_file_name.clone()), updated_veh);

    fs::read_dir(src_dir.clone()).unwrap().for_each(|entry| {
        let fname = entry.unwrap().file_name();
        let new_fname = fname.clone().to_str().unwrap().replace("skin", skin_file_name.as_str());

        fs::copy(src_dir.join(fname), out_dir.join(new_fname));
    });
}