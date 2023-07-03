use crate::os_methods;
use std::path::PathBuf;
use slint::SharedString;

slint::include_modules!();

pub fn run_main_window() {
  let main_window = Demo::new().unwrap();
  
  main_window.on_choose_dir({ let weak_main_window = main_window.as_weak(); move || {
    let ws_folder = os_methods::pick_folder().unwrap_or_default();
    weak_main_window.unwrap().invoke_change_dir(SharedString::from(ws_folder.clone()));
    if os_methods::validate_workshop_folder(&PathBuf::from(ws_folder)) {
      weak_main_window.unwrap().invoke_dir_validated();
    }
  }});

  main_window.on_validate_dir({ let weak_main_window = main_window.as_weak(); move |dirpath| {
    if os_methods::validate_workshop_folder(&PathBuf::from(dirpath.as_str())) {
      weak_main_window.unwrap().invoke_dir_validated();
    }
  }});

  main_window.run().unwrap()
}