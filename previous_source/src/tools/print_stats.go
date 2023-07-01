package tools

import (
	"../config"
	"fmt"
)

func PrintStats(conf config.ApplicationConfiguration) {
	
	fmt.Printf("### DRIVERS IN CONFIG: %d ###\n", len(conf.LeagueConfig.Driver))
	for _, driver := range conf.LeagueConfig.Driver {
		println(driver.Name)
	}
	println("")
	println("")
}