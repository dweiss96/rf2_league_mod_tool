use std::io::Error;
use std::path::Path;
use std::sync::mpsc::Sender;
use crate::models::car::Car;
use crate::models::league::League;
use crate::models::VersionInfo;
use crate::tasks::internal;

pub fn fill_mas_files(tmp_dir: &str, mod_mgr_path: &str, league:League, car: Car, sender: Sender<String>) {
    let car_files_path = Path::new(tmp_dir).join(car.id.clone()).join("*");
    let mas_file_path = Path::new(tmp_dir).join(car.id.clone()).join(format!("{}_{}.mas", car.id, league.version_prefix));

    internal::run_process_with_piped_output_and_wait(mod_mgr_path, vec!["-q", format!("-m\"{}\"", mas_file_path.to_str().unwrap()).as_str(), car_files_path.to_str().unwrap()], sender)
}

pub fn generate_rfcmp_files(tmp_dir: &str, mod_mgr_path: &str, car: Car, sender: Sender<String>) {
    let cmpinfo_path = Path::new(tmp_dir).join(car.id.clone()).join("cmpinfo.dat");

    internal::run_process_with_piped_output_and_wait(mod_mgr_path, vec!["-q", format!("-b\"{}\"", cmpinfo_path.to_str().unwrap()).as_str(), "0"], sender)
}

pub fn write_cmp_info(tmp_dir: &str, version: VersionInfo, league:League, car: Car) -> Result<(), Error>{
    let car_update_version = version.base_version.incremented();
    let update_version_string = format!("{}_{}_v{}",
                                        car_update_version.encode_to_string(),
                                        league.version_prefix,
                                        version.modpack_version.encode_to_string()
    );

    let out = Path::new(tmp_dir).join(car.id.clone()).join("cmpinfo.dat");
    let rfcmp_name = format!("{}_v{}.rfcmp", car.id, update_version_string);

    let content = format!(r#"[Component]
Name={}
Type=2
Version={}
BaseVersion={}
MinVersion=
Author=
Date=0
ID=
URL=
Desc=
Category=3
Origin=0
Flags=1
CurLocation=0
NumLocations=1
Location={}
NumMASFiles=1
MASFile={}
rFmFile=
IconFile=
SmallIconFile=

"#,
                          car.id.clone(),
                          update_version_string,
                          version.base_version.encode_to_string(),
                          out.with_file_name(rfcmp_name).to_str().unwrap(),
                          out.with_file_name(format!("{}_{}.mas", car.id, league.version_prefix)).to_str().unwrap()
    );
    
    std::fs::write(out, content.as_bytes())
}