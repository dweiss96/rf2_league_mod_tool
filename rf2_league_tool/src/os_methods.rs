use std::path::Path;

use glob::glob;
use rfd::FileDialog;

pub fn pick_file_with_validation(expected_filename: &str) -> Option<String> {
    let file = FileDialog::new()
        .add_filter("application", &["exe"])
        .add_filter("rust", &["rs", "toml"])
        .set_directory("..")
        .pick_file();

    let file_matches_name = file
        .as_ref()
        .filter(|f| {
            f.file_name()
                .and_then(|n| n.to_str().map(|s| s.eq(expected_filename)))
                .unwrap_or(false)
        })
        .is_some();

    if file_matches_name {
        file.map(|f| f.as_path().to_str().unwrap().to_string())
    } else {
        None
    }
}

pub fn pick_folder() -> Option<String> {
    FileDialog::new()
        .set_directory("..")
        .pick_folder()
        .and_then(|f| f.as_path().to_str().map(|s| s.to_string()))
}

pub fn validate_workshop_folder(folder: &Path) -> bool {
    glob(format!("{}/*/*.rfcmp", folder.to_str().unwrap_or("")).as_str())
        .expect("Failed to read glob pattern")
        .count()
        > 0
}
