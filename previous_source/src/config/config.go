package config

import (
	"fmt"
	"strings"
	"os"
	"io/ioutil"
	"encoding/json"
	"errors"
)

var AppConf ApplicationConfiguration
var ModMgrPath string
var PackVersion string
var Verbose bool

type ApplicationConfiguration struct {
    RF2Path    string `json:"rf2Path"`
	WorkshopPath  string `json:"workshopPath"`
	CarConfig []CarConfigModel `json:"cars"`
	LeagueConfig LeagueConfigModel `json:"league"`
	LeagueFiles []string `json:"leagueFiles"`
	CarFiles []string `json:"carFiles"`
	RequiredDriverFiles []string `json:"requiredDriverFiles"`
	OptionalDriverFiles []string `json:"optionalDriverFiles"`
}

func (c *ApplicationConfiguration) FindCar(id string) (CarConfigModel, error) {
	for _, car := range c.CarConfig {
		if(car.Id == id) {
			return car, nil
		}
	}

	return c.CarConfig[0], errors.New("No Car Found")
}

type CarConfigModel struct {
    Id    string `json:"id"`
	WorkshopID  string `json:"workshopId"`
	VersionOverwrite string `json:"versionOverwrite"`
	Name  string `json:"modName"`
	Postfixes  []string `json:"possiblePostfixes"`
}

type LeagueConfigModel struct {
	Name    string `json:"name"`
	VersionPrefix string `json:"versionPrefix"`
	TestDriver  []DriverModel `json:"testDriver"`
	Driver  []DriverModel `json:"driver"`
}

type DriverModel struct {
	Name string `json:"name"`
	Team string `json:"team"`
	Number string `json:"number"`
	Car string `json:"car"`
}

func (d *DriverModel) GetLiveryIdentifier() string {
	id := fmt.Sprintf("%sDAR", d.Number)
	return strings.ToUpper(id)
}

func ReadConfigFromJson(path string) ApplicationConfiguration {
	jsonFile, _ := os.Open(path)
	byteValue, _ := ioutil.ReadAll(jsonFile)
	var conf ApplicationConfiguration

	err := json.Unmarshal(byteValue, &conf)
	if(err != nil) {
		panic(err)
	}

	return conf
}

