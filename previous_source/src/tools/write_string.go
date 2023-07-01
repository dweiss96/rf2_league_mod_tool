package tools

import (
	"io/ioutil"
)

func WriteStringToFile(path string, content string) {
	contentBytes := []byte(content)
	err := ioutil.WriteFile(path, contentBytes, 0644)
	if(err != nil) {
		println(err.Error())
	}
}