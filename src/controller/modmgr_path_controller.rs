use crate::os_methods;
use std::path::PathBuf;
use slint::{ComponentHandle, SharedString};
use crate::slint_generatedMain::*;

pub fn initialize(window: Main) {
  window.global::<Rfactor2PathController>().on_choose_dir({ let ww = window.as_weak(); move || {
    let ws_folder = os_methods::pick_file_with_validation("ModMgr.exe").unwrap_or_default();

    ww.unwrap().global::<Rfactor2PathController>().set_path(SharedString::from(ws_folder.clone()));
    ww.unwrap().global::<Rfactor2PathController>().set_is_valid(os_methods::validate_workshop_folder(&PathBuf::from(ws_folder)));
  }});

  window.global::<Rfactor2PathController>().on_path_accepted({ let ww = window.as_weak(); move |mgrpath| {
    ww.unwrap().global::<Rfactor2PathController>().set_is_valid(
      mgrpath.ends_with("ModMgr.exe")
    )
  }});
}
