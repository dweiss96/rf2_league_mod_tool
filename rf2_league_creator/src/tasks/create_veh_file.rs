use crate::models::league::League;
use crate::models::driver::Driver;

#[cfg(test)]
mod tests {
    use super::*;
    fn get_league_and_driver() -> (League, Driver) {
        let driver = Driver {
            name: String::from("Max Mustermann"),
            team: String::from("Mustermann Racing"),
            number: 42,
            car: String::from("CAR_GT3_2023")
        };

        let league = League {
            name: String::from("Test Liga 23 S1"),
            car_class: String::from("TestLiga-23S1"),
            car_category: String::from("TestLiga, TestLiga 23 Saison 1"),
            upgrades_file_name: String::from("TestLigaUpgrades.ini"),
            livery_file_prefix: None,
            livery_file_suffix: Some(String::from("TST")),
            version_prefix: String::from("TST23"),
            cars: Vec::new(),
            drivers: Vec::from([driver.clone()])
        };

        return (league, driver)
    }

    #[test]
    fn substitutes_correctly_all_fields() {
        let target = r#"
            DefaultLivery="42TST.dds"
            MaterialOverride="AMV_GT3_BaseMat.json"

            HDVehicle=AM_V-GT3.hdv
            Graphics=AMV_GT3.gen
            Spinner=AMV_GT3_Spinner.gen
            Upgrades=TestLigaUpgrades.ini
            GenString=B                    // Used to generate GMT names in *.gen file
            Cameras=AMV_GT3.cam          // Defaults to cams.cfg in UserData directory
            Sounds=AMV_GT3.sfx          // Sounds=default.sfx
            HeadPhysics=HeadPhysics_GT.ini // Affects driver eyepoint only
            Cockpit=AMV_GT3_cockpitinfo.ini
            AIUpgradeClass=AMV-GT3
            ExternalDisplays=AMV_GT3_externalDisplay.ini

            //////////////////////////TEAM HISTORY AND INFORMATION///////////////////////////////////////////

            Number=42
            Team="Mustermann Racing"
            PitGroup="Group1"
            Driver="Max Mustermann"
            Description=" #42 Max Mustermann"
            Engine="AM 4.0L V6"
            Manufacturer="Aston Martin"
            Classes="TestLiga-23S1, AstonMartin_Vantage_GT3_2019"

            FullTeamName="Mustermann Racing"
            TeamHeadquarters=""

            Category="TestLiga, TestLiga 23 Saison 1, Aston Martin Vantage GT3"
        "#;

        let input = r#"
            DefaultLivery="{{LIVERY}}.dds"
            MaterialOverride="AMV_GT3_BaseMat.json"

            HDVehicle=AM_V-GT3.hdv
            Graphics=AMV_GT3.gen
            Spinner=AMV_GT3_Spinner.gen
            Upgrades={{UPGRADES}}
            GenString=B                    // Used to generate GMT names in *.gen file
            Cameras=AMV_GT3.cam          // Defaults to cams.cfg in UserData directory
            Sounds=AMV_GT3.sfx          // Sounds=default.sfx
            HeadPhysics=HeadPhysics_GT.ini // Affects driver eyepoint only
            Cockpit=AMV_GT3_cockpitinfo.ini
            AIUpgradeClass=AMV-GT3
            ExternalDisplays=AMV_GT3_externalDisplay.ini

            //////////////////////////TEAM HISTORY AND INFORMATION///////////////////////////////////////////

            Number={{NUMBER}}
            Team="{{TEAMNAME}}"
            PitGroup="Group1"
            Driver="{{DRIVER}}"
            Description=" #{{NUMBER_DESC}} {{DRIVER}}"
            Engine="AM 4.0L V6"
            Manufacturer="Aston Martin"
            Classes="{{CLASS}}, AstonMartin_Vantage_GT3_2019"

            FullTeamName="{{TEAMNAME}}"
            TeamHeadquarters=""

            Category="{{CATEGORY}}, Aston Martin Vantage GT3"
        "#;

        let league_and_driver = get_league_and_driver();
        assert_eq!(substitute_veh_fields(input, league_and_driver.0, league_and_driver.1), target);
    }
    
    #[test]
    fn substitutes_correctly_with_extra_fields_in_template() {
        let target = r#"
            DefaultLivery="42TST.dds"
            MaterialOverride="AMV_GT3_BaseMat.json"

            HDVehicle=AM_V-GT3.hdv
            Graphics=AMV_GT3.gen
            Spinner=AMV_GT3_Spinner.gen
            Upgrades=TestLigaUpgrades.ini
            GenString=B                    // Used to generate GMT names in *.gen file
            Cameras=AMV_GT3.cam          // Defaults to cams.cfg in UserData directory
            Sounds=AMV_GT3.sfx          // Sounds=default.sfx
            HeadPhysics=HeadPhysics_GT.ini // Affects driver eyepoint only
            Cockpit=AMV_GT3_cockpitinfo.ini
            AIUpgradeClass=AMV-GT3
            ExternalDisplays=AMV_GT3_externalDisplay.ini

            //////////////////////////TEAM HISTORY AND INFORMATION///////////////////////////////////////////

            Number=42
            Team="Mustermann Racing"
            PitGroup="{{GROUP}}"
            Driver="Max Mustermann"
            Description=" #42 Max Mustermann"
            Engine="AM 4.0L V6"
            Manufacturer="{{Manufacturer}}"
            Classes="TestLiga-23S1, AstonMartin_Vantage_GT3_2019"

            FullTeamName="Mustermann Racing"
            TeamHeadquarters=""

            Category="TestLiga, TestLiga 23 Saison 1, Aston Martin Vantage GT3"
        "#;

        let input = r#"
            DefaultLivery="{{LIVERY}}.dds"
            MaterialOverride="AMV_GT3_BaseMat.json"

            HDVehicle=AM_V-GT3.hdv
            Graphics=AMV_GT3.gen
            Spinner=AMV_GT3_Spinner.gen
            Upgrades={{UPGRADES}}
            GenString=B                    // Used to generate GMT names in *.gen file
            Cameras=AMV_GT3.cam          // Defaults to cams.cfg in UserData directory
            Sounds=AMV_GT3.sfx          // Sounds=default.sfx
            HeadPhysics=HeadPhysics_GT.ini // Affects driver eyepoint only
            Cockpit=AMV_GT3_cockpitinfo.ini
            AIUpgradeClass=AMV-GT3
            ExternalDisplays=AMV_GT3_externalDisplay.ini

            //////////////////////////TEAM HISTORY AND INFORMATION///////////////////////////////////////////

            Number={{NUMBER}}
            Team="{{TEAMNAME}}"
            PitGroup="{{GROUP}}"
            Driver="{{DRIVER}}"
            Description=" #{{NUMBER_DESC}} {{DRIVER}}"
            Engine="AM 4.0L V6"
            Manufacturer="{{Manufacturer}}"
            Classes="{{CLASS}}, AstonMartin_Vantage_GT3_2019"

            FullTeamName="{{TEAMNAME}}"
            TeamHeadquarters=""

            Category="{{CATEGORY}}, Aston Martin Vantage GT3"
        "#;

        let league_and_driver = get_league_and_driver();
        assert_eq!(substitute_veh_fields(input, league_and_driver.0, league_and_driver.1), target);
    }
    
    #[test]
    fn substitutes_correctly_with_fields_missing_in_template() {
        let target = r#"
            DefaultLivery="42TST.dds"
            MaterialOverride="AMV_GT3_BaseMat.json"

            HDVehicle=AM_V-GT3.hdv
            Graphics=AMV_GT3.gen
            Spinner=AMV_GT3_Spinner.gen
            Upgrades=UpgradesPreSet.ini
            GenString=B                    // Used to generate GMT names in *.gen file
            Cameras=AMV_GT3.cam          // Defaults to cams.cfg in UserData directory
            Sounds=AMV_GT3.sfx          // Sounds=default.sfx
            HeadPhysics=HeadPhysics_GT.ini // Affects driver eyepoint only
            Cockpit=AMV_GT3_cockpitinfo.ini
            AIUpgradeClass=AMV-GT3
            ExternalDisplays=AMV_GT3_externalDisplay.ini

            //////////////////////////TEAM HISTORY AND INFORMATION///////////////////////////////////////////

            Number=42
            Team="Mustermann Racing"
            PitGroup="Group1"
            Driver="Max Mustermann"
            Description=" #69 Max Mustermann"
            Engine="AM 4.0L V6"
            Manufacturer="Aston Martin"
            Classes="TestLiga-23S1, AstonMartin_Vantage_GT3_2019"

            FullTeamName="Mustermann Racing"
            TeamHeadquarters=""

            Category="TestLiga, TestLiga 23 Saison 1, Aston Martin Vantage GT3"
        "#;

        let input = r#"
            DefaultLivery="{{LIVERY}}.dds"
            MaterialOverride="AMV_GT3_BaseMat.json"

            HDVehicle=AM_V-GT3.hdv
            Graphics=AMV_GT3.gen
            Spinner=AMV_GT3_Spinner.gen
            Upgrades=UpgradesPreSet.ini
            GenString=B                    // Used to generate GMT names in *.gen file
            Cameras=AMV_GT3.cam          // Defaults to cams.cfg in UserData directory
            Sounds=AMV_GT3.sfx          // Sounds=default.sfx
            HeadPhysics=HeadPhysics_GT.ini // Affects driver eyepoint only
            Cockpit=AMV_GT3_cockpitinfo.ini
            AIUpgradeClass=AMV-GT3
            ExternalDisplays=AMV_GT3_externalDisplay.ini

            //////////////////////////TEAM HISTORY AND INFORMATION///////////////////////////////////////////

            Number={{NUMBER}}
            Team="{{TEAMNAME}}"
            PitGroup="Group1"
            Driver="{{DRIVER}}"
            Description=" #69 {{DRIVER}}"
            Engine="AM 4.0L V6"
            Manufacturer="Aston Martin"
            Classes="{{CLASS}}, AstonMartin_Vantage_GT3_2019"

            FullTeamName="{{TEAMNAME}}"
            TeamHeadquarters=""

            Category="{{CATEGORY}}, Aston Martin Vantage GT3"
        "#;
    
        let league_and_driver = get_league_and_driver();
        assert_eq!(substitute_veh_fields(input, league_and_driver.0, league_and_driver.1), target);
    }
    
}

pub fn substitute_veh_fields(template: &str, league_config: League, driver: Driver) -> String {
    return template
        .replace("{{CLASS}}", &league_config.car_class)
        .replace("{{CATEGORY}}", &league_config.car_category)
        .replace("{{UPGRADES}}", &league_config.upgrades_file_name)
        .replace("{{LIVERY}}", &format!("{}{}{}", league_config.livery_file_prefix.clone().unwrap_or_default(), driver.number, league_config.livery_file_suffix.clone().unwrap_or_default()))
        .replace("{{NUMBER}}", &format!("{}", driver.number))
        .replace("{{NUMBER_DESC}}", &format!("{}", driver.number))
        .replace("{{DRIVER}}", &driver.name)
        .replace("{{TEAMNAME}}", &driver.team);
}