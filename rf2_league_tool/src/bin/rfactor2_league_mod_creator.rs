extern crate rf2_league_tool;

use crate::rf2_league_tool::*;

fn main() {
    // memory allocation in main method since they need to be accessible for the whole time
    let generator_thread: ThreadHandle = ThreadHandle::empty();
    let output_thread: ThreadHandle = ThreadHandle::empty();

    view::initialize_main_window(generator_thread, output_thread)
        .run()
        .unwrap();

    // as per rusts handling of threads and child processes, we are safe on exit of
    // the main thread and rely on the OS to kill all children
}
