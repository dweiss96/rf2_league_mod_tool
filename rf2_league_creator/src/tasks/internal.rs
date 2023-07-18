use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;
use std::{
    io::{BufRead, BufReader},
    thread,
};
use crate::error::*;

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
) -> Result<(), CaughtError> {
    let mut child = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .catch_err()?;

    let stdout = child
        .stdout
        .take()
        .catch_none("Could not connect to StdOut".to_string())?;

    let reader = BufReader::new(stdout);

    let output = thread::spawn({
        let s = sender;
        move || {
            reader
                .lines()
                .map_while(|line| line.ok())
                .for_each(|line| s.send(line).unwrap_or(()));
        }
    });

    let res = child.wait();
    output.join().catch_err_with_msg("could not join output child process".to_string())?;
    res.catch_err().map(|_| {})
}