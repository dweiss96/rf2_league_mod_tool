use std::thread;
use std::sync::mpsc;
use crate::ThreadHandle;
use slint::{ComponentHandle, SharedString};

use rf2_league_creator::generate_mod;

use crate::slint_generatedMain::*;

unsafe impl Send for Main {}
unsafe impl Sync for Main {}

pub fn initialize(main: Main, generator_thread: ThreadHandle, output_thread: ThreadHandle) {
  main.global::<GeneratorState>().set_current_state(GeneratorStates::Ready);
  main.global::<GeneratorState>().set_current_state_description(SharedString::from("Waiting for orders"));

  let (tx, rx) = mpsc::channel::<String>();
  main.global::<GeneratorState>().on_start({ let ww = main.as_weak(); move || {
    ww.upgrade().unwrap().global::<GeneratorState>().set_output_log(SharedString::from(""));
    ww.upgrade().unwrap().global::<GeneratorState>().set_current_state(GeneratorStates::CopyFiles);
    ww.upgrade().unwrap().global::<GeneratorState>().set_current_state_description(SharedString::from("Copying Files"));

    let local_generator_thread = thread::spawn({ let txx = tx.clone(); move || {
      generate_mod(
        ww.unwrap().global::<ConfigurationState>().get_data().into(), "temp", "1.0", "target", txx
      );
      // generate_mod_with_json_default("temp", "1.0", "target", txx).ok().unwrap_or(());
    }});

    generator_thread.lock().unwrap().replace(local_generator_thread);
  }});
  //
  // main.global::<GeneratorState>().on_cancel({ let ww = main.as_weak(); move || {
  //   unsafe {
  //     generator_thread.as_mut().unwrap_or(&mut None).take().and_then(|thread| {
  //       thread.join();
  //       None::<()>
  //     });
  //   }
  //   ww.upgrade().unwrap().global::<GeneratorState>().set_current_state(GeneratorStates::Failed);
  //   ww.upgrade().unwrap().global::<GeneratorState>().set_current_state_description(SharedString::from("Manually Canceled"));
  // }});

  let weak_main = main.as_weak();
  let local_output_thread = thread::spawn(move || {
    loop {
      if let Ok(line) = rx.recv() {
        let handle_copy = weak_main.clone();
        slint::invoke_from_event_loop(move || handle_copy.unwrap().global::<GeneratorState>().invoke_add_log_line(line.into())).unwrap_or(());
      }
    }
  });
  output_thread.lock().unwrap().replace(local_output_thread);
}
