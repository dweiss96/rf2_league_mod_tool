use std::path::Path;
use crate::os_methods;
use slint::{ComponentHandle, SharedString, ModelRc, VecModel, Model};

use rf2_league_creator::models::Config;

use crate::slint_generatedMain::*;

fn read_config(path: &Path, conf_state: ConfigurationState) {
  Config::read_from(path).map(|config| {
    let mut gui_config: Configuration = config.into();
    gui_config.workshop_found = os_methods::validate_workshop_folder(Path::new(gui_config.workshop_path.as_str()));
    gui_config.modmgr_found = gui_config.modmgr_path.ends_with("ModMgr.exe");
    conf_state.set_data(gui_config)
  }).unwrap_or_else(|_| {
    conf_state.set_data(Configuration {
      modmgr_found: false,
      modmgr_path: SharedString::new(),
      workshop_found: false,
      workshop_path: SharedString::new(),
      league_config: LeagueConfiguration { 
        cars: ModelRc::new(VecModel::from(vec![])),
        car_names: ModelRc::new(VecModel::from(vec![])),
        driver: ModelRc::new(VecModel::from(vec![])),
        category: SharedString::new(),
        class: SharedString::new(),
        livery_file_prefix: SharedString::new(),
        livery_file_suffix: SharedString::new(),
        name: SharedString::new(),
        upgrade_file: SharedString::new(),
        version_prefix: SharedString::new(),
      },
    })
  });
}

pub fn initialize(main: Main) {
  let config_path = Path::new("config.json");

  main.global::<ConfigurationState>().on_read_config({let wm = main.as_weak(); move || {
    read_config(config_path, wm.unwrap().global::<ConfigurationState>())
  }});

  main.global::<ConfigurationState>().on_save_config({let wm = main.as_weak(); move || {
    let config: Config = wm.unwrap().global::<ConfigurationState>().get_data().into();
    let _ = config.write_to(config_path);
  }});

  main.global::<ConfigurationState>().on_modmgr_choose_dir({ let wm = main.as_weak(); move || {
    let path = os_methods::pick_file_with_validation("ModMgr.exe").unwrap_or_default();
    let mut config = wm.unwrap().global::<ConfigurationState>().get_data();
    
    config.modmgr_found = path.ends_with("ModMgr.exe");
    config.modmgr_path = SharedString::from(path);

    wm.unwrap().global::<ConfigurationState>().set_data(config);
  }});

  main.global::<ConfigurationState>().on_workshop_choose_dir({ let wm = main.as_weak(); move || {
    let path = os_methods::pick_folder().unwrap_or_default();
    let mut config = wm.unwrap().global::<ConfigurationState>().get_data();
  
    config.workshop_found = os_methods::validate_workshop_folder(Path::new(path.as_str()));
    config.workshop_path = SharedString::from(path);

    wm.unwrap().global::<ConfigurationState>().set_data(config);
  }});

  main.global::<ConfigurationState>().on_add_car({ let wm = main.as_weak(); move |id, workshop, version| {
    let cars = wm.unwrap().global::<ConfigurationState>().get_data().league_config.cars;
    let cars_model = cars.as_any().downcast_ref::<VecModel<CarConfiguration>>().unwrap();
    cars_model.push(CarConfiguration {
      id: id.clone(),
      workshop_id: workshop,
      version_overwrite: version
    });

    let car_names = wm.unwrap().global::<ConfigurationState>().get_data().league_config.car_names;
    let car_names_model = car_names.as_any().downcast_ref::<VecModel<SharedString>>().unwrap();
    car_names_model.push(id);
  }});

  main.global::<ConfigurationState>().on_delete_car({ let wm = main.as_weak(); move |index| {
    let cars = wm.unwrap().global::<ConfigurationState>().get_data().league_config.cars;
    let cars_model = cars.as_any().downcast_ref::<VecModel<CarConfiguration>>().unwrap();
    cars_model.remove(index.to_string().parse::<usize>().unwrap());

    let car_names = wm.unwrap().global::<ConfigurationState>().get_data().league_config.car_names;
    let car_names_model = car_names.as_any().downcast_ref::<VecModel<SharedString>>().unwrap();
    car_names_model.remove(index.to_string().parse::<usize>().unwrap());
  }});

  main.global::<ConfigurationState>().on_move_car({ let wm = main.as_weak(); move |index, movement| {
    let cars = wm.unwrap().global::<ConfigurationState>().get_data().league_config.cars;
    let cars_model = cars.as_any().downcast_ref::<VecModel<CarConfiguration>>().unwrap();

    let car_names = wm.unwrap().global::<ConfigurationState>().get_data().league_config.car_names;
    let car_names_model = car_names.as_any().downcast_ref::<VecModel<SharedString>>().unwrap();
    if (index+movement) >= 0 && (index+movement) < cars_model.row_count().to_string().parse::<i32>().unwrap() {
      let parsed_index = index.to_string().parse::<usize>().unwrap();
      let new_index = (index+movement).to_string().parse::<usize>().unwrap();
      let car = cars_model.remove(parsed_index);
      cars_model.insert(new_index, car);

      let car_name = car_names_model.remove(parsed_index);
      car_names_model.insert(new_index, car_name);
    }
  }});

  main.global::<ConfigurationState>().on_update_car_version({ let wm = main.as_weak(); move |index, new_version| {
    let cars: ModelRc<CarConfiguration> = wm.unwrap().global::<ConfigurationState>().get_data().league_config.cars;
    let cars_model = cars.as_any().downcast_ref::<VecModel<CarConfiguration>>().unwrap();

    let parsed_index = index.to_string().parse::<usize>().unwrap();
    
    let mut car = cars_model.remove(parsed_index);
    car.version_overwrite = new_version;
    
    cars_model.insert(parsed_index, car);
  }});

  main.global::<ConfigurationState>().on_add_driver({ let wm = main.as_weak(); move |name, number, car, team| {
    let drivers = wm.unwrap().global::<ConfigurationState>().get_data().league_config.driver;
    let driver_model = drivers.as_any().downcast_ref::<VecModel<DriverConfiguration>>().unwrap();
    driver_model.push(DriverConfiguration {
      name,
      number: number.parse::<i32>().unwrap(),
      car,
      team,
    });
  }});

  main.global::<ConfigurationState>().on_delete_driver({ let wm = main.as_weak(); move |index| {
    let drivers = wm.unwrap().global::<ConfigurationState>().get_data().league_config.driver;
    let driver_model = drivers.as_any().downcast_ref::<VecModel<DriverConfiguration>>().unwrap();
    driver_model.remove(index.to_string().parse::<usize>().unwrap());
  }});

  main.global::<ConfigurationState>().on_move_driver({ let wm = main.as_weak(); move |index, movement| {
    let drivers = wm.unwrap().global::<ConfigurationState>().get_data().league_config.driver;
    let driver_model = drivers.as_any().downcast_ref::<VecModel<DriverConfiguration>>().unwrap();
    if (index+movement) >= 0 && (index+movement) < driver_model.row_count() as i32 {
      let parsed_index = index.to_string().parse::<usize>().unwrap();
      let new_index = (index+movement).to_string().parse::<usize>().unwrap();
      let driver = driver_model.remove(parsed_index);
      driver_model.insert(new_index, driver);
    }
  }});

  main.global::<ConfigurationState>().on_update_driver_car({ let wm = main.as_weak(); move |index, new_car| {
    let drivers = wm.unwrap().global::<ConfigurationState>().get_data().league_config.driver;
    let driver_model = drivers.as_any().downcast_ref::<VecModel<DriverConfiguration>>().unwrap();

    let parsed_index = index.to_string().parse::<usize>().unwrap();
    
    let mut driver = driver_model.remove(parsed_index);
    driver.car = new_car;
    
    driver_model.insert(parsed_index, driver);
  }});

  main.global::<ConfigurationState>().on_update_driver_team({ let wm = main.as_weak(); move |index, new_team| {
    let drivers = wm.unwrap().global::<ConfigurationState>().get_data().league_config.driver;
    let driver_model = drivers.as_any().downcast_ref::<VecModel<DriverConfiguration>>().unwrap();

    let parsed_index = index.to_string().parse::<usize>().unwrap();
    
    let mut driver = driver_model.remove(parsed_index);
    driver.team = new_team;
    
    driver_model.insert(parsed_index, driver);
  }});

  read_config(config_path, main.global::<ConfigurationState>())
}
