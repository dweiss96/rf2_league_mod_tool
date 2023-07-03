use crate::os_methods;
use std::path::PathBuf;
use slint::ComponentHandle;
use slint::SharedString;
use crate::slint_generatedMain::*;

pub fn initialize(window: Main) {
  window.global::<WorkshopPathController>().on_choose_dir({ let ww = window.as_weak(); move || {
    let ws_folder = os_methods::pick_folder().unwrap_or_default();

    ww.unwrap().global::<WorkshopPathController>().set_path(SharedString::from(ws_folder.clone()));
    ww.unwrap().global::<WorkshopPathController>().set_is_valid(os_methods::validate_workshop_folder(&PathBuf::from(ws_folder)));
  }});

  window.global::<WorkshopPathController>().on_path_accepted({ let ww = window.as_weak(); move |dirpath| {
    ww.unwrap().global::<WorkshopPathController>().set_is_valid(
      os_methods::validate_workshop_folder(&PathBuf::from(dirpath.as_str()))
    )
  }});
}
