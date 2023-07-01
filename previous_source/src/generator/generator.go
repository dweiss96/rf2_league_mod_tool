package generator

import (
	"fmt"
	"os"

	"../config"
)

func generateDirForCar(id string) error {
	return os.MkdirAll(fmt.Sprintf("./_generated_outputs/%s/files", id), os.ModeDir)
}

func RemoveOutputDir() {
	err := os.RemoveAll(fmt.Sprintf("./_generated_outputs"))
	if err != nil {
		println(err.Error())
	}
}

func Generate(debug bool) {
	for _, car := range config.AppConf.CarConfig {
		err := generateDirForCar(car.Id)
		if err != nil {
			fmt.Printf("ERROR CREATE_DIR FOR CAR %s \n", car.Id)
		}
	}

	for _, driver := range config.AppConf.LeagueConfig.TestDriver {
		copyDriverFilesToDir(driver, true)
	}
	for _, driver := range config.AppConf.LeagueConfig.Driver {
		copyDriverFilesToDir(driver, false)
	}

	for _, car := range config.AppConf.CarConfig {
		copyCarFilesToDir(car)
		if debug {
			generateDebugRF2Files(car)
		} else {
			generateRF2Files(car)
		}
	}
	
	copyLeagueFiles()
}
