package helpers

import (
	"fmt"
	"log"
	"os"
	"strings"
)

func CreateDirectory(path string) {
	split := strings.Split(path, "/")
	file := split[len(split)-1]
	fmt.Println(file)
	if err := os.MkdirAll(path, os.ModePerm); err != nil {
		log.Fatal("Error on create directory:", err)
	}
}
