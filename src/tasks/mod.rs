use std::process::{Child, Command, Stdio};
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;
use std::{
    io::{BufRead, BufReader},
    thread,
};

use crate::models::driver::Driver;
use crate::models::league::League;
use tempfile::tempdir;

automod::dir!("src/tasks");

pub struct ProcessHandle {
    child: Child,
    sender: Sender<Option<String>>,
    output_handle: JoinHandle<()>,
}

impl ProcessHandle {
    // pub fn wait(mut self) {
    //     _ = self.child.wait();
    //     _ = self.output_handle.join().unwrap_or_default();
    //     self.sender.send(None).unwrap_or_default();
    // }
    pub fn kill(mut self) {
        _ = self.child.kill();
        _ = self.output_handle.join().unwrap();
        self.sender.send(None).unwrap();
    }
}

///
/// Function to run a process and handle it's output.
/// You should always wait() on success or kill() the returned handle.
///
/// # Arguments
///
/// * `command`: command to be run. enter as it would be used in a terminal
/// * `args`: arguments for that command. each space would be a new arg
/// * `sender`: output consuming sender mpsc channel
///
/// returns: ProcessHandle to interact with the threads
///
pub fn run_process_with_output(
    command: &str,
    args: Vec<&str>,
    sender: Sender<Option<String>>,
) -> ProcessHandle {
    let mut child = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "Could not connect to StdOut")
        .unwrap();
    let reader = BufReader::new(stdout);

    let output_handle = thread::spawn({
        let s = sender.clone();
        move || {
            reader
                .lines()
                .filter_map(|line| line.ok())
                .for_each(|line| s.send(Some(line)).unwrap_or_default());
        }
    });

    ProcessHandle {
        child,
        sender,
        output_handle,
    }
}

pub fn generate(league: League, driver: Driver) {
    // Create a directory inside of `std::env::temp_dir()`.
    let tmp_dir = tempdir().unwrap();

    copy_files::copy("", tmp_dir.path().to_str().unwrap(), league, driver);

    // By closing the `TempDir` explicitly, we can check that it has
    // been deleted successfully. If we don't close it explicitly,
    // the directory will still be deleted when `dir` goes out
    // of scope, but we won't know whether deleting the directory
    // succeeded.
    tmp_dir.close().unwrap();
}
