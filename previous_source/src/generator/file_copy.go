package generator

import(
	"fmt"
	"strings"
	"../config"
	"../tools"
)

func copyDriverFilesToDir(driver config.DriverModel, isTestDriver bool) {
	skinPath := fmt.Sprintf("./templates/%s/skins/%s", driver.Car, driver.Number)
	outputPath := fmt.Sprintf("./_generated_outputs/%s/files", driver.Car)

	for _, skinFile := range config.AppConf.RequiredDriverFiles {
		newSkinFilePrefix := strings.Replace(skinFile, "skin", driver.GetLiveryIdentifier(), 1)
		inputFile := fmt.Sprintf("%s/%s", skinPath, skinFile)
		outputFile := fmt.Sprintf("%s/%s", outputPath, newSkinFilePrefix)
		_,copyErr := tools.CopyFile(inputFile, outputFile)
		if(copyErr != nil) {
			panic(copyErr)
		}
	}
	for _, skinFile := range config.AppConf.OptionalDriverFiles {
		newSkinFilePrefix := strings.Replace(skinFile, "skin", driver.GetLiveryIdentifier(), 1)
		inputFile := fmt.Sprintf("%s/%s", skinPath, skinFile)
		outputFile := fmt.Sprintf("%s/%s", outputPath, newSkinFilePrefix)
		_, copyErr := tools.CopyFile(inputFile, outputFile)
		if(copyErr != nil && config.Verbose) {
			fmt.Printf("Error copying optional file %s for %s #%s\n", skinFile, driver.Car, driver.Number)
		}
	}
	
	vehTemplatePath := fmt.Sprintf("./templates/%s/_vehicle.veh", driver.Car)
	vehOutputPath := fmt.Sprintf("%s/%s.veh", outputPath, driver.GetLiveryIdentifier())
	vehErr := fillInVehFile(vehTemplatePath, vehOutputPath, driver, isTestDriver)
	if(vehErr != nil) {
		panic(vehErr)
	}
}

func copyLeagueFiles() {
	skinPath := fmt.Sprintf("./templates/_league")
	outputPath := fmt.Sprintf("./_generated_outputs")

	for _, skinFile := range config.AppConf.LeagueFiles  {
		inputFile := fmt.Sprintf("%s/%s", skinPath, skinFile)
		outputFile := fmt.Sprintf("%s/%s", outputPath, skinFile)
		_,copyErr := tools.CopyFile(inputFile, outputFile)
		if(copyErr != nil) {
			panic(copyErr)
		}
	}
}

func copyCarFilesToDir(car config.CarConfigModel) {
	skinPath := fmt.Sprintf("./templates/%s", car.Id)
	outputPath := fmt.Sprintf("./_generated_outputs/%s/files", car.Id)

	for _, skinFile := range config.AppConf.CarFiles  {
		inputFile := fmt.Sprintf("%s/%s", skinPath, skinFile)
		outputFile := fmt.Sprintf("%s/%s", outputPath, skinFile)
		_,copyErr := tools.CopyFile(inputFile, outputFile)
		if(copyErr != nil) {
			panic(copyErr)
		}
	}
}