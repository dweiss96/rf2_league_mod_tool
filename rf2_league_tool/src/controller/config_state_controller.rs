use std::path::Path;
use crate::os_methods;
use slint::{ComponentHandle, SharedString, ModelRc, VecModel};

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

  read_config(config_path, main.global::<ConfigurationState>())
}
