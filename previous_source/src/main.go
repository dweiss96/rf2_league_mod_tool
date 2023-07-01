package main

import (
	"flag"
	"fmt"
	"time"
	"os"
	"./config"
	"./tools"
	"./generator"
)

var verbosePtr = flag.Bool("v", false, "be verbose")
var timePtr = flag.Bool("t", false, "meassure time")
var debugPtr = flag.Bool("debug", false, "does NOT create MAS and RFCMP files")
var versionPtr = flag.String("version", "1.0", "version number for the livery pack")
var idPtr = flag.Bool("id_gen", false, "print the `id` of the cars from the config file")
var carVersionPtr = flag.Bool("car_versions", false, "print the `id` and `version` of the cars from the workshop folder")
var statPtr = flag.Bool("stats", false, "print statistics of the config json")

var programPwd string

func main() {
	flag.Parse()
	
	programPwd, _ = os.Getwd()

	if(*timePtr) {
		currentTime := time.Now()
		tools.WriteStringToFile(fmt.Sprintf("%s/startTime.log", programPwd), fmt.Sprintf("############ TIME MEASSURE START ############\n%s\n", currentTime.Format("2006-01-02 15:04:05")))
	}

	config.AppConf = config.ReadConfigFromJson("./config.json")
	config.ModMgrPath = fmt.Sprintf("%s\\Bin64\\ModMgr.exe", config.AppConf.RF2Path)
	config.Verbose = *verbosePtr
	config.PackVersion = *versionPtr


	
	if(*carVersionPtr) {
		tools.PrintCarIdsAndVersion(config.AppConf.CarConfig)
		return
	}
	if(*idPtr) {
		tools.PrintCarIds(config.AppConf.CarConfig)
		return
	}
	if(*statPtr) {
		tools.PrintStats(config.AppConf)
		return
	}

	generator.RemoveOutputDir()
	generator.Generate(*debugPtr)

	if(*timePtr) {
		currentTime := time.Now()
		tools.WriteStringToFile(fmt.Sprintf("%s/endTime.log", programPwd), fmt.Sprintf("############ TIME MEASSURE END ############\n%s\n", currentTime.Format("2006-01-02 15:04:05")))
	}
}
