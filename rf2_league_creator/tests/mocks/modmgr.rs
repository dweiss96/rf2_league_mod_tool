use std::env;
use std::path::Path;

fn main() {
    // check for `-q` flag
    assert!(env::args().fold(false, |acc, f| {acc || f.as_str() == "-q"}), "needs to be executed quietly");
    let mut mas_path:String = String::new();
    let mut cmpinfodat_path:String = String::new();

    for argument in env::args().skip(1) {
        if argument.starts_with("-q") {
            // do nothing on q flag but it is required
            println!("quiet")
        } else if argument.eq("0") {
            // do nothing on 0 argument but it is required
            println!("zero")
        } else if argument.starts_with("-b") {
            // handle RFCMP build command

            // assert existing cmpinfo.dat
            cmpinfodat_path.push_str(argument.trim_start_matches("-b\"").trim_end_matches("\""));
            assert!(Path::new(cmpinfodat_path.as_str()).file_name().unwrap().to_str().unwrap().ends_with("cmpinfo.dat"), "dat path needs to end with the correct file name `cmpinfo.dat`");

            // read information from the cmpinfo.dat
            let cmpinfo = String::from_utf8(std::fs::read(Path::new(cmpinfodat_path.as_str())).unwrap()).unwrap();
            let rfcmp_loc = cmpinfo.split_once("\nLocation=").unwrap().1.split_once("NumMASFiles").unwrap().0.trim();
            let mas_loc = cmpinfo.split_once("\nMASFile=").unwrap().1.split_once("rFmFile").unwrap().0.trim();
            let mas_content = String::from_utf8(std::fs::read(Path::new(mas_loc)).unwrap()).unwrap();

            // create test validation information
            let mut rfcmp_content = String::new();
            rfcmp_content.push_str(format!("\n### CMPINFO ###\n{cmpinfo}\n").as_str());
            rfcmp_content.push_str(format!("\n### MASFILE ###\n{mas_content}\n").as_str());

            // write file for final test assertion
            std::fs::write(Path::new(cmpinfodat_path.as_str()).parent().unwrap().join(rfcmp_loc), rfcmp_content).unwrap();

            println!("rfcmp");
        } else if argument.starts_with("-m") {
            // handle MAS build command

            // assert mas is given as expected
            mas_path.push_str(argument.trim_start_matches("-m\"").trim_end_matches("\""));
            assert!(Path::new(mas_path.as_str()).file_name().unwrap().to_str().unwrap().ends_with(".mas"), "mas path needs to end with the correct extension");

            // read files for test assertion
            let mut mas_content = "".to_string();
            let files = std::fs::read_dir(Path::new(mas_path.as_str()).parent().unwrap()).unwrap();
            files.map(|r| r.unwrap()).for_each(|e| {
                if e.file_name().to_str().unwrap().ends_with(".veh") {
                    // .veh files is copied as text since these are modified by the tool
                    mas_content.push_str(format!("\nVEHFILE {}", e.file_name().to_str().unwrap()).as_str());
                    mas_content.push_str("\n######");
                    mas_content.push_str(format!("\n{}", String::from_utf8(std::fs::read(e.path()).unwrap()).unwrap()).as_str());
                    mas_content.push_str("\n######");
                } else {
                    // all other files are just renamed and content is of no interest
                    mas_content.push_str(format!("\nFILE {}", e.file_name().to_str().unwrap()).as_str());
                }
            });

            // write mas file for later validation
            std::fs::write(Path::new(mas_path.as_str()), mas_content).unwrap();

            println!("mas");
        } else {
            // assert directory to not be empty from which the files are read
            assert!(std::fs::read_dir(Path::new(argument.as_str()).parent().unwrap()).unwrap().count() > 1, "The source directory should have more than one file since UPGRADES.ini and at least one .veh File are necessary.");

            println!("source");
        }
    }

    // one last println to ensure no panic in the process
    println!("generated");
}