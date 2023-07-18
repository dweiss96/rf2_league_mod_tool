use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;
use std::{
    io::{BufRead, BufReader},
    thread,
};

///
/// Function to run a process and send it's output while running.
///
/// # Arguments
///
/// * `command`: command to be run. enter as it would be used in a terminal
/// * `args`: arguments for that command. each space would be a new arg
/// * `sender`: output consuming sender mpsc channel
///
/// returns: JoinHanlde to interact with the thread
///
pub fn run_process_with_piped_output_and_wait(
    command: &str,
    args: Vec<&str>,
    sender: Sender<String>,
) {
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

    let output = thread::spawn({
        let s = sender.clone();
        move || {
            reader
                .lines()
                .filter_map(|line| line.ok())
                .for_each(|line| s.send(line).unwrap_or_default());
        }
    });

    let _ = child.wait().unwrap();
    output.join().unwrap()
}