package generator

import (
	"fmt"
	"io/ioutil"
	"os"
	"os/exec"
	"strconv"
	"strings"

	"../config"
	"../resources"
	"../tools"
)

func findComponentInformation(carConfig config.CarConfigModel) ComponentInfo {
	workshopPath := fmt.Sprintf("%s\\%s", config.AppConf.WorkshopPath, carConfig.WorkshopID)
	filesInWorkshop := tools.GetFileList(workshopPath)

	workshopFilePrefixMatcher := fmt.Sprintf("%s\\%s", workshopPath, carConfig.Id)

	rfcmpFiles := tools.FilterString(filesInWorkshop, func(in string) bool { return strings.HasSuffix(in, ".rfcmp") })
	carRfcmpFiles := tools.FilterString(rfcmpFiles, func(in string) bool { return strings.HasPrefix(in, workshopFilePrefixMatcher) })

	highestEvenVersion := VersionInfo{
		0,
		0,
	}

	
	if(carConfig.VersionOverwrite == "")  {
		for _, file := range carRfcmpFiles {
			fileWithoutExt := strings.Replace(file, ".rfcmp", "", 1)
			fileWithoutCarPrefix := strings.Replace(fileWithoutExt, workshopFilePrefixMatcher, "", 1)
			versionNumber := strings.Replace(fileWithoutCarPrefix, "_v", "", 1)
			versionParts := strings.Split(versionNumber, ".")
	
			minorVersion, minorErr := strconv.Atoi(versionParts[1])
			majorVersion, majorErr := strconv.Atoi(versionParts[0])
	
			if minorErr != nil {
				panic(minorErr)
			}
			if majorErr != nil {
				panic(majorErr)
			}
	
			if minorVersion%2 == 0 {
				if majorVersion > highestEvenVersion.Major {
					highestEvenVersion = VersionInfo{
						minorVersion,
						majorVersion,
					}
				} else if majorVersion == highestEvenVersion.Major && minorVersion > highestEvenVersion.Minor {
					highestEvenVersion = VersionInfo{
						minorVersion,
						majorVersion,
					}
				}
			}
		}
	} else {
		versionParts := strings.Split(carConfig.VersionOverwrite, ".")
		minorVersion, minorErr := strconv.Atoi(versionParts[1])
		majorVersion, majorErr := strconv.Atoi(versionParts[0])

		if minorErr != nil {
			panic(minorErr)
		}
		if majorErr != nil {
			panic(majorErr)
		}
		highestEvenVersion = VersionInfo{
			minorVersion,
			majorVersion,
		}
	}

	dir, _ := os.Getwd()

	updateVersionString := fmt.Sprintf("%s%s_v%s", highestEvenVersion.UpdateVersionString(), config.AppConf.LeagueConfig.VersionPrefix, config.PackVersion)

	outputPath := fmt.Sprintf("%s\\_generated_outputs\\%s_v%s.rfcmp", dir, carConfig.Id, updateVersionString)
	inputPath := fmt.Sprintf("%s\\_generated_outputs\\%s\\%s_LEAGUE.mas", dir, carConfig.Id, carConfig.Id)

	return ComponentInfo{
		highestEvenVersion,
		carConfig.Id,
		outputPath,
		inputPath,
	}
}

/*func generateMASFile(carId string) {
	pwd, _ := os.Getwd()
	carFilesPath := fmt.Sprintf("%s\\_generated_outputs\\%s\\files", pwd, carId)
	masPath := fmt.Sprintf("%s\\_generated_outputs\\%s\\%s_LEAGUE.mas", pwd, carId, carId)

	for _, file := range tools.GetFileList(carFilesPath) {
		if file != carFilesPath {
			command := fmt.Sprintf("& '%s' -q -m\"%s\" '%s'", config.ModMgrPath, masPath, file)
			cmd := exec.Command("powershell", "-NoExit", command)
			_, addErr := cmd.CombinedOutput()
			if addErr != nil {
				println(addErr.Error())
				panic(addErr)
			}
			println(command)
		}
	}
}*/

func generateMASFile(carId string) {
	pwd, _ := os.Getwd()
	carFilesPath := fmt.Sprintf("%s\\_generated_outputs\\%s\\files\\*", pwd, carId)
	masPath := fmt.Sprintf("%s\\_generated_outputs\\%s\\%s_LEAGUE.mas", pwd, carId, carId)

	command := fmt.Sprintf("& '%s' -q -m\"%s\" '%s'", config.ModMgrPath, masPath, carFilesPath)
	cmd := exec.Command("powershell", "-NoExit", command)
	_, addErr := cmd.CombinedOutput()
	if addErr != nil {
		println(addErr.Error())
		panic(addErr)
	}
	println(command)
}

func packageComponent(carId string) {
	pwd, _ := os.Getwd()
	cdErr := os.Chdir(config.AppConf.RF2Path)
	if cdErr != nil {
		panic(cdErr)
	}

	datPath := fmt.Sprintf("%s\\_generated_outputs\\%s\\cmpinfo.dat", pwd, carId)
	command := fmt.Sprintf("& '%s' -q -b\"%s\" 0", config.ModMgrPath, datPath)
	println(command)
	_, packageErr := exec.Command("powershell", "-NoExit", command).CombinedOutput()

	// go back to old working dir to be able to read files
	backCdErr := os.Chdir(pwd)
	if backCdErr != nil {
		panic(backCdErr)
	} // is more important than error on adding files since it has implications for other cars

	if packageErr != nil {
		println(packageErr.Error())
		panic(packageErr)
	}
}

type VersionInfo struct {
	Minor int
	Major int
}

func (v *VersionInfo) VersionString() string {
	if v.Minor == 0 {
		return fmt.Sprintf("%d.00", v.Major)
	}
	return fmt.Sprintf("%d.%d", v.Major, v.Minor)
}

func (v *VersionInfo) UpdateVersionString() string {
	if v.Minor == 0 {
		return fmt.Sprintf("%d.01", v.Major)
	}
	return fmt.Sprintf("%d.%d", v.Major, v.Minor+1)
}

type ComponentInfo struct {
	Base      VersionInfo
	Name      string
	OutputDir string
	InputMAS  string
}

func writeCmpInfo(carConfig config.CarConfigModel) {
	outputPath := fmt.Sprintf("./_generated_outputs/%s/cmpinfo.dat", carConfig.Id)

	cmpInfo := findComponentInformation(carConfig)

	content := resources.ComponentDatFile

	updateVersionString := fmt.Sprintf("%s%s_v%s", cmpInfo.Base.UpdateVersionString(), config.AppConf.LeagueConfig.VersionPrefix, config.PackVersion)

	content = strings.ReplaceAll(content, "{{CAR}}", carConfig.Id)
	content = strings.ReplaceAll(content, "{{UPDATE_VERSION}}", updateVersionString)
	content = strings.ReplaceAll(content, "{{BASE_VERSION}}", cmpInfo.Base.VersionString())
	content = strings.ReplaceAll(content, "{{OUTPUT_DIR}}", cmpInfo.OutputDir)
	content = strings.ReplaceAll(content, "{{MAS_FILE}}", cmpInfo.InputMAS)
	writeError := ioutil.WriteFile(outputPath, []byte(content), 0644)
	if writeError != nil {
		panic(writeError)
	}
}

func generateRF2Files(carConfig config.CarConfigModel) {
	writeCmpInfo(carConfig)
	generateMASFile(carConfig.Id)
	packageComponent(carConfig.Id)
}

func generateDebugRF2Files(carConfig config.CarConfigModel) {
	writeCmpInfo(carConfig)
}
