package helpers

import (
	"fmt"
	"log"
	"os"
	"strings"
)

func CreateFile(path string, template *Template) *os.File {
	path = "src/" + path
	directories := strings.Split(path, "/")
	totalItems := len(directories)
	filename := directories[totalItems-1]
	pathWithoutFile := directories[:totalItems-1]

	name := Capitalize(filename)

	createdDirs := ""

	for _, directory := range pathWithoutFile {
		createdDirs = createdDirs + directory + "/"
		if _, err := os.Stat(createdDirs); os.IsNotExist(err) {
			if err = os.Mkdir(createdDirs, os.ModePerm); err != nil {
				log.Fatal("Error to create directory", err)
			}
		}
	}

	filename = filename + template.Sufix + ".ts"

	file, err := os.Create(createdDirs + filename)

	if err != nil {
		fmt.Println("Error on create file:", err)
	}
	defer file.Close()

	content, err := TemplateContent(template.Type, name+template.Name)

	if err != nil {
		log.Fatal("Error get template content", err)
	}

	_, err = file.WriteString(content)

	if err != nil {
		log.Fatal("Error to write file", err)
	}

	file.Sync()
	return file
}
