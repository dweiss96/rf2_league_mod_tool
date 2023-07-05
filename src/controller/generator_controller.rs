use std::thread;
use std::ops::Add;
use std::sync::mpsc;
use slint::{ComponentHandle, SharedString};

use crate::tasks::run_process_with_output;

use crate::slint_generatedMain::*;

pub fn initialize(window: Main) {
  window.global::<GeneratorState>().set_current_state(GeneratorStates::Ready);
  window.global::<GeneratorState>().set_current_state_description(SharedString::from("Waiting for orders"));
  window.global::<GeneratorState>().on_start({ let weak_window = window.as_weak(); move || {
    let (tx, rx) = mpsc::channel();
    let local_process_handle = run_process_with_output("ping", vec!["-c","10","bigbl4ckw0lf.de"], tx.clone());

    let eof_generator = thread::spawn(|| {
      let _ = &local_process_handle.wait();
    });

    loop {
      match rx.recv() {
        Ok(Some(line)) => {
          let previous_log = weak_window.unwrap().global::<GeneratorState>().get_output_log();
          let new_log = previous_log.add("\n").add(line.as_str());
          println!("{}", new_log);
          weak_window.unwrap().global::<GeneratorState>().set_output_log(new_log);
          weak_window.unwrap().global::<GeneratorState>().set_current_state(GeneratorStates::CopyFiles);
          weak_window.unwrap().global::<GeneratorState>().set_current_state_description(SharedString::from("Copying Files"));
        },
        Ok(None) => break,
        _ => {}
      }
    }

    eof_generator.join().unwrap_or_default();

    weak_window.unwrap().global::<GeneratorState>().set_current_state(GeneratorStates::Finished);
    weak_window.unwrap().global::<GeneratorState>().set_current_state_description(SharedString::from("Finished Task"));
  }});
  //
  // window.global::<GeneratorState>().on_cancel({ let ww = window.as_weak(); let ohr = output_handle.as_ref(); move || {
  //   match (process_handle, &output_handle) {
  //     (Some(ph), Some(oh)) => ph.kill_with(&oh),
  //     (Some(ph), _) => ph.kill(),
  //     _ => {}
  //   }
  // }});

  // window.global::<GeneratorState>().on_start({ let ww = window.as_weak(); move || {
  //   let ws_folder = os_methods::pick_folder().unwrap_or_default();

  //   ww.unwrap().global::<GeneratorState>().set_path(SharedString::from(ws_folder.clone()));
  //   ww.unwrap().global::<GeneratorState>().set_is_valid(os_methods::validate_workshop_folder(&PathBuf::from(ws_folder)));
  // }});
}
