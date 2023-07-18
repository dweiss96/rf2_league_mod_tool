extern crate rf2_league_creator;

use getopts::Options;
use tempfile::tempdir;

#[cfg(not(tarpaulin_include))]
fn main() {
    let td = tempdir().unwrap();
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "out", "set output directory", "OUT_DIR");
    opts.optopt("v", "version", "set mod version", "VERSION");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };
    if matches.opt_present("h") {
        print!("{}", opts.usage(&format!("Usage: {} [options]", program)));
        return;
    }
    let output = matches.opt_str("o").unwrap_or(std::env::current_dir().unwrap().join("output").to_str().unwrap().to_string());
    let version = matches.opt_str("v").unwrap_or("1.0".to_string());

    let (tx, rx) = std::sync::mpsc::channel::<String>();

    let _ = std::thread::spawn(move || loop {
        match rx.recv() {
            Ok(line) => println!("rf2cli: {}", line),
            _ => {}
        }
    });

    rf2_league_creator::generate_mod_with_json_default(
        td.path().to_str().unwrap(),
        version.as_str(),
        output.as_str(),
        tx,
    );
    td.close().unwrap();
}
