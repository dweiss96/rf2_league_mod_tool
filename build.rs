use std::env;
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("{}", out_dir);
    slint_build::compile("ui/main.slint").unwrap();
}
