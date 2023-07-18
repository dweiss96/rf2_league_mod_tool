use rf2_league_creator::models::car::Car;
use rf2_league_creator::models::driver::Driver;
use rf2_league_creator::models::league::League;
use rf2_league_creator::models::paths::Paths;
use rf2_league_creator::models::*;
use std::path::{Path, PathBuf};
use std::thread;

#[cfg(test)]
mod common;

fn get_config(cmd_path: &PathBuf, ws_path: &PathBuf, vo: Option<String>) -> Config {
    Config {
        paths: Paths {
            modmgr: cmd_path.to_str().unwrap().to_string(),
            rf2: "".to_string(),
            workshop: ws_path.to_str().unwrap().to_string(),
        },
        league: get_league_cars_and_drivers(vo).0,
    }
}

fn get_league_cars_and_drivers(vo: Option<String>) -> (League, Vec<Car>, Vec<Driver>) {
    let drivers = vec![
        Driver {
            name: String::from("Max Mustermann"),
            team: String::from("M&M Racing"),
            number: 42,
            car: String::from("CAR_GT3_2023"),
        },
        Driver {
            name: String::from("Thomas Müller"),
            team: String::from("M&M Racing"),
            number: 69,
            car: String::from("CAR_GT3_2023"),
        },
        Driver {
            name: String::from("John Stone"),
            team: String::from("Ipsum Racing"),
            number: 13,
            car: String::from("CAR_GTE_2023"),
        },
        Driver {
            name: String::from("Jackie Meldrum"),
            team: String::from("Ipsum Racing"),
            number: 7,
            car: String::from("CAR_GTE_2023"),
        },
    ];

    let cars = vec![
        Car {
            id: "CAR_GT3_2023".to_string(),
            workshop_id: "123456789".to_string(),
            version_overwrite: vo.clone(),
        },
        Car {
            id: "CAR_GTE_2023".to_string(),
            workshop_id: "012345678".to_string(),
            version_overwrite: vo,
        },
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
        drivers: drivers.clone(),
    };

    (league, cars, drivers)
}

#[test]
fn it_can_setup_tests() {
    let td = common::modmgr_setup();

    let cmd_path = td.path().join(common::get_modmgr_filename());
    let ws_path = td.path().join(".workshop");
    assert!(!cmd_path.to_str().unwrap_or_default().is_empty());

    let cfg_fixed = get_config(&cmd_path, &ws_path, Some("1.00".to_string()));
    let cfg_search = get_config(&cmd_path, &ws_path, None);

    assert_eq!(cfg_fixed.league.version_prefix, "TST23");
    assert_eq!(cfg_search.league.version_prefix, "TST23");

    td.close().unwrap();
    assert!(true)
}

#[cfg(test)]
fn assert_equal_files(td: &str, src: impl AsRef<Path>, dst: impl AsRef<Path>) {
    let src_content = std::fs::read(src).unwrap();
    let dst_content = std::fs::read(dst).unwrap();
    pretty_assertions::assert_eq!(
        String::from_utf8_lossy(src_content.as_slice()).replace(td, "TEMP"),
        String::from_utf8_lossy(dst_content.as_slice()).replace(td, "TEMP")
    );
}

#[test]
fn it_works_as_expected_with_config_struct_and_fixed_version() {
    let td = common::modmgr_setup();

    // generate modmgr path
    let cmd_path = td.path().join(common::get_modmgr_filename());
    let ws_path = td.path().join(".workshop");
    std::fs::create_dir_all(Path::new(td.path().to_str().unwrap()).join(".out")).unwrap();

    assert!(!cmd_path.to_str().unwrap_or_default().is_empty());

    // generate config
    let cfg = get_config(&cmd_path, &ws_path, Some("1.00".to_string()));

    // generate needed variables
    let (tx, rx) = std::sync::mpsc::channel();
    let mut output_log: Vec<String> = Vec::new();

    // run test case as separate thread
    let test_handle = thread::spawn({
        let tdd = td.path().to_str().unwrap().to_string();
        move || {
            rf2_league_creator::generate_mod(
                cfg,
                tdd.as_str(),
                "1.0",
                Path::new(tdd.as_str()).join(".out").to_str().unwrap(),
                tx.clone(),
            )
            .unwrap();
        }
    });

    // read output lines parallel
    while let Ok(line) = rx.recv() {
        output_log.push(line)
    }

    // get all back to sequential execution
    test_handle.join().unwrap();

    let mas_commands = "quiet;mas;source;generated";
    let rfcmp_commands = "quiet;rfcmp;zero;generated";

    // assert correctly read output log (indicates correct order)
    assert_eq!(
        output_log.join(";"),
        format!(
            "{};{};{};{}",
            mas_commands, rfcmp_commands, mas_commands, rfcmp_commands
        )
    );

    // // Debug tmp dir after generation
    // println!("{}", td.path().to_str().unwrap());
    // thread::sleep(std::time::Duration::from_secs(60));

    let files_in_out: Vec<String> = Vec::from_iter(
        std::fs::read_dir(td.path().join(".out"))
            .unwrap()
            .map(|er| er.unwrap().file_name().to_str().unwrap().to_string()),
    );

    assert!(vec![
        "CAR_GTE_2023_v1.00.rfcmp",
        "CAR_GT3_2023_v1.00.rfcmp",
        "CAR_GT3_2023_v1.01_TST23_v1.00.rfcmp",
        "CAR_GTE_2023_v1.01_TST23_v1.00.rfcmp"
    ]
    .iter()
    .map(|e| { files_in_out.contains(&e.to_string()) })
    .fold(true, |acc, v| { acc && v }));

    assert_equal_files(
        td.path().to_str().unwrap(),
        td.path()
            .join(".out")
            .join("CAR_GT3_2023_v1.01_TST23_v1.00.rfcmp"),
        Path::new(std::env::var("CARGO_MANIFEST_DIR").unwrap().as_str())
            .join("tests")
            .join("expected_results")
            .join("v1.01_TST23_v1.00")
            .join("CAR_GT3_2023_v1.01_TST23_v1.00.rfcmp"),
    );
    assert_equal_files(
        td.path().to_str().unwrap(),
        td.path()
            .join(".out")
            .join("CAR_GTE_2023_v1.01_TST23_v1.00.rfcmp"),
        Path::new(std::env::var("CARGO_MANIFEST_DIR").unwrap().as_str())
            .join("tests")
            .join("expected_results")
            .join("v1.01_TST23_v1.00")
            .join("CAR_GTE_2023_v1.01_TST23_v1.00.rfcmp"),
    );

    // close temp dir
    td.close().unwrap();
}

#[test]
fn it_works_as_expected_with_config_struct_and_fixed_version_with_missing_workshop() {
    let td = common::modmgr_setup();

    // generate modmgr path
    let cmd_path = td.path().join(common::get_modmgr_filename());
    std::fs::create_dir_all(Path::new(td.path().to_str().unwrap()).join(".out")).unwrap();

    assert!(!cmd_path.to_str().unwrap_or_default().is_empty());

    // generate config
    let cfg = get_config(&cmd_path, &cmd_path, Some("1.00".to_string()));

    // generate needed variables
    let (tx, rx) = std::sync::mpsc::channel();
    let mut output_log: Vec<String> = Vec::new();

    // run test case as separate thread
    let test_handle = thread::spawn({
        let tdd = td.path().to_str().unwrap().to_string();
        move || {
            rf2_league_creator::generate_mod(
                cfg,
                tdd.as_str(),
                "1.0",
                Path::new(tdd.as_str()).join(".out").to_str().unwrap(),
                tx.clone(),
            )
            .unwrap();
        }
    });

    // read output lines parallel
    while let Ok(line) = rx.recv() {
        output_log.push(line)
    }

    // get all back to sequential execution
    test_handle.join().unwrap();

    let mas_commands = "quiet;mas;source;generated";
    let rfcmp_commands = "quiet;rfcmp;zero;generated";

    // assert correctly read output log (indicates correct order)
    assert_eq!(
        output_log.join(";"),
        format!(
            "{};{};{};{}",
            mas_commands, rfcmp_commands, mas_commands, rfcmp_commands
        )
    );

    // // Debug tmp dir after generation
    // println!("{}", td.path().to_str().unwrap());
    // thread::sleep(std::time::Duration::from_secs(60));

    let files_in_out: Vec<String> = Vec::from_iter(
        std::fs::read_dir(td.path().join(".out"))
            .unwrap()
            .map(|er| er.unwrap().file_name().to_str().unwrap().to_string()),
    );

    assert!(vec![
        "CAR_GTE_2023_v1.00.rfcmp",
        "CAR_GT3_2023_v1.00.rfcmp",
        "CAR_GT3_2023_v1.01_TST23_v1.00.rfcmp",
        "CAR_GTE_2023_v1.01_TST23_v1.00.rfcmp"
    ]
    .iter()
    .map(|e| { files_in_out.contains(&e.to_string()) })
    .fold(true, |acc, v| { acc && v }));

    assert_equal_files(
        td.path().to_str().unwrap(),
        td.path()
            .join(".out")
            .join("CAR_GT3_2023_v1.01_TST23_v1.00.rfcmp"),
        Path::new(std::env::var("CARGO_MANIFEST_DIR").unwrap().as_str())
            .join("tests")
            .join("expected_results")
            .join("v1.01_TST23_v1.00")
            .join("CAR_GT3_2023_v1.01_TST23_v1.00.rfcmp"),
    );
    assert_equal_files(
        td.path().to_str().unwrap(),
        td.path()
            .join(".out")
            .join("CAR_GTE_2023_v1.01_TST23_v1.00.rfcmp"),
        Path::new(std::env::var("CARGO_MANIFEST_DIR").unwrap().as_str())
            .join("tests")
            .join("expected_results")
            .join("v1.01_TST23_v1.00")
            .join("CAR_GTE_2023_v1.01_TST23_v1.00.rfcmp"),
    );

    // close temp dir
    td.close().unwrap();
}

#[test]
fn it_works_as_expected_with_config_struct_and_dynamic_version() {
    let td = common::modmgr_setup();

    // generate modmgr path
    let cmd_path = td.path().join(common::get_modmgr_filename());
    let ws_path = td.path().join(".workshop");
    std::fs::create_dir_all(Path::new(td.path().to_str().unwrap()).join(".out")).unwrap();

    assert!(!cmd_path.to_str().unwrap_or_default().is_empty());

    // generate config
    let cfg = get_config(&cmd_path, &ws_path, None);

    // generate needed variables
    let (tx, rx) = std::sync::mpsc::channel();
    let mut output_log: Vec<String> = Vec::new();

    // run test case as separate thread
    let test_handle = thread::spawn({
        let tdd = td.path().to_str().unwrap().to_string();
        move || {
            rf2_league_creator::generate_mod(
                cfg,
                tdd.as_str(),
                "1.0",
                Path::new(tdd.as_str()).join(".out").to_str().unwrap(),
                tx.clone(),
            )
            .unwrap();
        }
    });

    // read output lines parallel
    while let Ok(line) = rx.recv() {
        output_log.push(line)
    }

    // get all back to sequential execution
    test_handle.join().unwrap();

    let mas_commands = "quiet;mas;source;generated";
    let rfcmp_commands = "quiet;rfcmp;zero;generated";

    // assert correctly read output log (indicates correct order)
    assert_eq!(
        output_log.join(";"),
        format!(
            "{};{};{};{}",
            mas_commands, rfcmp_commands, mas_commands, rfcmp_commands
        )
    );

    // Debug tmp dir after generation
    // println!("{}", td.path().to_str().unwrap());
    // thread::sleep(std::time::Duration::from_secs(60));

    let files_in_out: Vec<String> = Vec::from_iter(
        std::fs::read_dir(td.path().join(".out"))
            .unwrap()
            .map(|er| er.unwrap().file_name().to_str().unwrap().to_string()),
    );

    assert!(vec![
        "CAR_GTE_2023_v1.00.rfcmp",
        "CAR_GT3_2023_v1.00.rfcmp",
        "CAR_GT3_2023_v1.03_TST23_v1.00.rfcmp",
        "CAR_GTE_2023_v1.03_TST23_v1.00.rfcmp"
    ]
    .iter()
    .map(|e| { files_in_out.contains(&e.to_string()) })
    .fold(true, |acc, v| { acc && v }));

    assert_equal_files(
        td.path().to_str().unwrap(),
        td.path()
            .join(".out")
            .join("CAR_GT3_2023_v1.03_TST23_v1.00.rfcmp"),
        Path::new(std::env::var("CARGO_MANIFEST_DIR").unwrap().as_str())
            .join("tests")
            .join("expected_results")
            .join("v1.03_TST23_v1.00")
            .join("CAR_GT3_2023_v1.03_TST23_v1.00.rfcmp"),
    );
    assert_equal_files(
        td.path().to_str().unwrap(),
        td.path()
            .join(".out")
            .join("CAR_GTE_2023_v1.03_TST23_v1.00.rfcmp"),
        Path::new(std::env::var("CARGO_MANIFEST_DIR").unwrap().as_str())
            .join("tests")
            .join("expected_results")
            .join("v1.03_TST23_v1.00")
            .join("CAR_GTE_2023_v1.03_TST23_v1.00.rfcmp"),
    );

    // close temp dir
    td.close().unwrap();
}
