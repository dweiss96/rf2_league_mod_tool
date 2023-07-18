use std::env;
use std::path::Path;
use std::process::Command;
use tempfile::{tempdir, TempDir};

#[cfg(not(tarpaulin_include))]
pub fn get_modmgr_filename() -> String {
    if env::consts::OS == "windows" {
        "modmgr.exe".to_string()
    } else {
        "modmgr".to_string()
    }
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) {
    std::fs::create_dir_all(&dst).unwrap();
    for entry in std::fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let ty = entry.file_type().unwrap();
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()));
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name())).unwrap();
        }
    }
}

pub fn modmgr_setup() -> TempDir {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR");
    let example_files_dir = Path::new(manifest_dir.clone().unwrap().as_str())
        .join("tests")
        .join("example_files");

    let td = tempdir().unwrap();
    let wsp = td.path().join(".workshop");
    std::fs::create_dir_all(wsp.clone()).unwrap();
    copy_dir_all(example_files_dir.join("workshop"), wsp);

    let modmgr_mock = Path::new(manifest_dir.clone().unwrap().as_str())
        .join("tests")
        .join("mocks")
        .join("modmgr.rs");

    let out_path = td.path().join(get_modmgr_filename());

    let _ = Command::new("rustc")
        .args(vec![
            modmgr_mock.as_path().to_str().unwrap(),
            "-o",
            out_path.to_str().unwrap(),
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // std::fs::read_dir(Path::new(manifest_dir.clone().unwrap().as_str())
    //                       .join("tests")
    //                       .join("example_files")
    //                       .join("workshop")).unwrap().map(|r| r.unwrap()).for_each(|f| {
    //     std::fs::copy(f.path(), wsp.clone().join(f.file_name())).unwrap();
    // });

    // set working directory to example files dir
    env::set_current_dir(example_files_dir).unwrap();

    td
}
