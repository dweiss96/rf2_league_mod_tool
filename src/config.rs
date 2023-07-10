pub fn _generator_command(modmgrpath: &str) -> String {
    format!("{}", modmgrpath)
}

pub fn _driver_files<'a>() -> (Vec<&'a str>, Vec<&'a str>) {
    (
        vec![
            "skin_region.dds",
            "skin.json",
            "skin_helmet.dds",
            "skin_driver.dds",
            "skindriver.dds",
            "skinDRIVER.dds",
            "skinextra0.dds",
            "skinextra2.dds",
            "skinEXTRA0.dds",
            "skinEXTRA1.dds",
            "skinEXTRA7.dds",
            "skinhelmet.dds",
            "skinHELMET.dds",
            "skinwindow.dds",
            "skin_windowsout.dds",
            "skinWINDOWSOUT.dds",
            "skinwindshieldout.dds",
            "skinWindshieldOut.dds",
            "skinWindshieldIn.dds",
            "skinicon.png",
            "skin-icon-2048x1152.png",
            "skin-icon-1024x576.png",
            "skin-icon-512x288.png",
            "skin-icon-256x144.png",
            "skin-icon-128x72.png",
        ],
        vec!["skin.dds"],
    )
}

fn _get_current_working_dir() -> String {
    std::env::current_dir()
        .unwrap()
        .as_path()
        .to_str()
        .unwrap()
        .to_string()
}
