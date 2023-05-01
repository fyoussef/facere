package helpers

import (
	"fmt"
	"log"
	"os"
	"strings"
)

func CreateDirectory(path string, template *Template) {
	path = "src/" + path
	split := strings.Split(path, "/")
	totalItems := len(split)

	filename := split[totalItems-1]

	className := Capitalize(path)

	err := os.WriteFile(filename, []byte(fmt.Sprintf("export class %s {}", className+template.Name)), os.ModePerm)

	if err != nil {
		fmt.Println("Error on write file", err)
	}

	path = strings.Join(split, "/")

	if err := os.MkdirAll(path, os.ModePerm); err != nil {
		log.Fatal("Error on create directory:", err)
	}

	// return CreateFile(file)
}
