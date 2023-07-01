package tools

import (
	"../config"
	"fmt"
	"strings"
)

func PrintCarIdsAndVersion(cars []config.CarConfigModel) {
	for _, car := range cars {
		workshopPath := fmt.Sprintf("%s\\%s", config.AppConf.WorkshopPath, car.WorkshopID)
		filesInWorkshop := GetFileList(workshopPath)
	
		rfcmpFiles := FilterString(filesInWorkshop, func(in string) bool { return strings.HasSuffix(in, ".rfcmp") })
	
		for _, file := range rfcmpFiles {
			fileWithoutExt := strings.Replace(file, ".rfcmp", "", 1)
			fileWithoutPathAndExt := strings.Replace(fileWithoutExt, workshopPath, "", 1)
			println(fileWithoutPathAndExt)
		}
	}
}

func PrintCarIds(cars []config.CarConfigModel) {
	for _, car := range cars {
		fmt.Printf("%s\n", car.Id)
	}
}