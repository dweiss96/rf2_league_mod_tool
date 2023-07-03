use crate::controller::workshop_path_controller;
use crate::controller::modmgr_path_controller;

use crate::slint_generatedMain::*;
use slint::ComponentHandle;

pub fn run_main_window() {
  let main_window = Main::new().unwrap();

  workshop_path_controller::initialize(main_window.clone_strong());
  // modmgr_path_controller::initialize(&main_window);

  // let fs = Main::ws_select::new().unwrap();
  
  // main_window.on_ws_choose_dir({ let weak_main_window = main_window.as_weak(); move || {
  //   let ws_folder = os_methods::pick_folder().unwrap_or_default();

  //   weak_main_window.unwrap().invoke_ws_change_path(SharedString::from(ws_folder.clone()));
  //   if os_methods::validate_workshop_folder(&PathBuf::from(ws_folder)) {
  //     weak_main_window.unwrap().invoke_ws_validate_path();
  //   }
  // }});

  // main_window.on_ws_path_accepted({ let weak_main_window = main_window.as_weak(); move |dirpath| {
  //   if os_methods::validate_workshop_folder(&PathBuf::from(dirpath.as_str())) {
  //     weak_main_window.unwrap().invoke_ws_validate_path();
  //   }
  // }});

  

  main_window.run().unwrap()
}