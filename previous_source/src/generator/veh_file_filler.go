package generator

import (
	"io/ioutil"
	"strings"
	"../config"
)

func fillInVehFile(templatePath string, outputPath string, driver config.DriverModel, isTestDriver bool) error {
	bytes, readError := ioutil.ReadFile(templatePath)
	if(readError != nil) {
		return readError
	}
	
	content := string(bytes)

	if(isTestDriver) {
		content = strings.ReplaceAll(content, "Description=\"#{{NUMBER_DESC}} {{DRIVER}}\"", "Description=\"#00 DAR\"")
	}	

	content = strings.ReplaceAll(content, "{{LIVERY}}", driver.GetLiveryIdentifier())
	content = strings.ReplaceAll(content, "{{NUMBER}}", driver.Number)
	content = strings.ReplaceAll(content, "{{NUMBER_DESC}}", strings.Split(driver.Number, "_")[0])
	content = strings.ReplaceAll(content, "{{DRIVER}}", driver.Name)
	content = strings.ReplaceAll(content, "{{TEAMNAME}}", driver.Team)
	content = strings.ReplaceAll(content, "{{EXTRA_CATEGORY}}", "")
	writeError := ioutil.WriteFile(outputPath, []byte(content), 0644)
	if(writeError != nil) {
		return writeError
	}
	return nil
}