use crate::os_methods;
use std::path::PathBuf;
use slint::SharedString;
use crate::slint_generatedMain::Main;
use slint::ComponentHandle;

pub fn initialize(window: &Main) {
  window.on_mgr_choose_dir({ let weak_main_window = window.as_weak(); move || {
    let ws_folder = os_methods::pick_file_with_validation("ModMgr.exe").unwrap_or_default();

    weak_main_window.unwrap().invoke_mgr_change_path(SharedString::from(ws_folder.clone()));
    if os_methods::validate_workshop_folder(&PathBuf::from(ws_folder)) {
      weak_main_window.unwrap().invoke_mgr_validate_path();
    }
  }});

  window.on_mgr_path_accepted({ let weak_main_window = window.as_weak(); move |dirpath| {
    if os_methods::validate_workshop_folder(&PathBuf::from(dirpath.as_str())) {
      weak_main_window.unwrap().invoke_mgr_validate_path();
    }
  }});
}
