package tools

import (
    "os"
    "path/filepath"
)

func GetFileList(path string) []string {
	var files []string

	root := path
	err := filepath.Walk(root, func(path string, info os.FileInfo, err error) error {
		files = append(files, path)
		return nil
	})
	if err != nil {
		panic(err)
	}

	return files
}