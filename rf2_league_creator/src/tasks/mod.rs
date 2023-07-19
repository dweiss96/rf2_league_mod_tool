use crate::error::*;
use crate::models::{Config, VersionInfo, VersionSpec};
use std::fs;
use std::path::Path;
use std::sync::mpsc::Sender;

automod::dir!("src/tasks");

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_reads_config() {
        let cfg = read_config("tests/example_files/config.json").unwrap();
        assert_eq!(cfg.league.name, "Test Liga 23 S1")
    }
}

pub fn read_config(cfg_path: &str) -> Result<Config, CaughtError> {
    let path = Path::new(cfg_path);
    let file = fs::read(path).catch_err()?;
    let content = String::from_utf8(file).catch_err()?;

    serde_json::from_str(content.as_str()).catch_err()
}

pub fn find_base_version(car_ws_id: String, ws_path: &Path) -> Result<VersionSpec, CaughtError> {
    let files = fs::read_dir(ws_path.join(car_ws_id)).catch_err()?;
    let base_version = files
        // .map(|r| r.unwrap())
        .map(|r| {
            let f = r.catch_err()?;
            if f.file_name()
                .to_str()
                .catch_none("could not decode filename for all files for base version".to_string())?
                .ends_with(".rfcmp")
            {
                let file_name = f.file_name();
                let version_name = file_name
                    .to_str()
                    .catch_none("could not decode filename for base version".to_string())?
                    .split_once('v')
                    .catch_none("could not decode version for base version".to_string())?
                    .1
                    .trim_end_matches(".rfcmp");
                let vc = VersionSpec::decode_from_str(version_name)?;
                if vc.minor % 2 == 0 {
                    Ok(vc)
                } else {
                    Ok(VersionSpec::empty())
                }
            } else {
                Ok(VersionSpec::empty())
            }
        })
        .find(|v| match v {
            Ok(version) => version.major > 0,
            _ => false,
        });

    base_version.unwrap_or(Ok(VersionSpec::empty()))
}

pub fn generate(
    config: Config,
    tmp_dir: &str,
    target_dir: &str,
    version: &str,
    tx: Sender<String>,
) -> Result<(), CaughtError> {
    // TODO: Create logos on the fly

    // copy all files to tmp working directory
    copy_files::copy(
        std::env::current_dir()
            .catch_err()?
            .to_str()
            .catch_none("could not decode current working directory".to_string())?,
        tmp_dir,
        config.league.clone(),
    )?;

    for car in config.league.clone().cars {
        let version_info = VersionInfo {
            base_version: car
                .clone()
                .version_overwrite
                .map(VersionSpec::decode_from_string)
                .unwrap_or_else(|| {
                    find_base_version(car.clone().workshop_id, Path::new(&config.paths.workshop))
                })?,
            modpack_version: VersionSpec::decode_from_str(version)?,
        };
        let _ = rfactor2::write_cmp_info(tmp_dir, version_info, config.league.clone(), car.clone());
        rfactor2::fill_mas_files(
            tmp_dir,
            config.paths.modmgr.as_str(),
            config.league.clone(),
            car.clone(),
            tx.clone(),
        )?;
        rfactor2::generate_rfcmp_files(
            tmp_dir,
            config.paths.modmgr.as_str(),
            car.clone(),
            tx.clone(),
        )?;
    }

    // copy results to target dir
    fs::read_dir(tmp_dir)
        .catch_err()?
        .map(|er| {
            let e = er.catch_err()?;
            if e.file_type().catch_err()?.is_dir() && e.path().ne(Path::new(target_dir)) {
                fs::read_dir(e.path())
                    .catch_err()?
                    .map(|entry| {
                        let file_name = entry.catch_err()?.file_name();
                        let file_name_string = file_name
                            .to_str()
                            .catch_none("could not decode file name to string".to_string())?;
                        if file_name_string.ends_with(".rfcmp") {
                            fs::copy(
                                e.path().join(file_name_string),
                                Path::new(target_dir).join(file_name_string),
                            )
                            .catch_err()
                            .map(|_| {})
                        } else {
                            Ok(())
                        }
                    })
                    .find(|r| r.is_err())
                    .unwrap_or(Ok(()))
            } else {
                let file_name = e.file_name();
                let file_name_string = file_name
                    .to_str()
                    .catch_none("could not decode file name to string".to_string())?;
                if file_name_string.ends_with(".rfcmp") {
                    fs::copy(e.path(), Path::new(target_dir).join(file_name_string))
                        .catch_err()
                        .map(|_| {})
                } else {
                    Ok(())
                }
            }
        })
        .find(|r| r.is_err())
        .unwrap_or(Ok(()))
}
