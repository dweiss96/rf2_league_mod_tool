use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc;
use slint::{ComponentHandle, SharedString};

use crate::tasks::{ProcessHandle, run_process_with_output};

use crate::slint_generatedMain::*;

unsafe impl Send for Main {}
unsafe impl Sync for Main {}

pub fn initialize(main: Main, generator_thread: *mut Option<ProcessHandle>, output_thread: *mut Option<JoinHandle<()>>) {
  main.global::<GeneratorState>().set_current_state(GeneratorStates::Ready);
  main.global::<GeneratorState>().set_current_state_description(SharedString::from("Waiting for orders"));

  let (tx, rx) = mpsc::channel::<Option<String>>();

  main.global::<GeneratorState>().on_start({ let ww = main.as_weak(); move || {
    ww.upgrade().expect("\n\ncould not unwrap weak window on start 1\n\n").global::<GeneratorState>().set_output_log(SharedString::from(""));
    ww.upgrade().expect("\n\ncould not unwrap weak window on start 2\n\n").global::<GeneratorState>().set_current_state(GeneratorStates::CopyFiles);
    ww.upgrade().expect("\n\ncould not unwrap weak window on start 3\n\n").global::<GeneratorState>().set_current_state_description(SharedString::from("Copying Files"));

    let local_process_handle = run_process_with_output("ping", vec!["-c","10","bigbl4ckw0lf.de"], tx.clone());

    unsafe {
      println!("unsafe `generator_thread` access to set");
      generator_thread.replace(Some(local_process_handle));
    }
  }});

  main.global::<GeneratorState>().on_cancel({ let ww = main.as_weak(); move || {
    unsafe {
      generator_thread.as_mut().unwrap_or(&mut None).take().and_then(|thread| {
        thread.kill();
        None::<()>
      });
    }
    ww.upgrade().expect("\n\ncould not unwrap weak window on cancel 1\n\n").global::<GeneratorState>().set_current_state(GeneratorStates::Failed);
    ww.upgrade().expect("\n\ncould not unwrap weak window on cancel 2\n\n").global::<GeneratorState>().set_current_state_description(SharedString::from("Manually Canceled"));
  }});

  let local_output_thread = thread::spawn(move || {
    loop {
      match rx.recv() {
        Ok(Some(line)) => {
          println!("{}", line);
          // let previous_log = main.global::<GeneratorState>().get_output_log();
          // let _new_log = previous_log.add("\n").add(line.as_str());
          // let state = main.global::<GeneratorState>().set_output_log(SharedString::from(line));
        },
        Ok(None) => break,
        _ => {}
      }
    }
  });

  unsafe {
    println!("unsafe `output_thread` access to set");
    output_thread.replace(Some(local_output_thread));
  }
}
