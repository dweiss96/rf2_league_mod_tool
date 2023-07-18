use crate::models::Config;
use std::fs;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_reads_config() {
        let cfg = read_config("tests/example_files/config.json");
        assert_eq!(cfg.league.name, "Test Liga 23 S1")
    }
}

pub fn read_config(cfg_path: &str) -> Config {
    let path = Path::new(cfg_path);
    let file = fs::read(path);
    let content = String::from_utf8(file.unwrap()).unwrap();

    serde_json::from_str(content.as_str()).unwrap()
}
