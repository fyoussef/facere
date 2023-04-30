package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"

	"github.com/fyoussef/scafolding.git/helpers"
)

func main() {
	terminalReader := bufio.NewReader(os.Stdin)

	fmt.Print("Digite o nome do arquivo: ")
	line, _, err := terminalReader.ReadLine()

	if err != nil {
		fmt.Println("Error to read terminal:", err)
	}

	commands := string(line[:])

	args := strings.Fields(commands)

	template := templates(args[1])

	name := args[2]

	if name == "" {
		log.Fatal("Filename has missing")
	}

	filename := name + template.sufix
	file, err := os.Create(filename + ".ts")

	if err != nil {
		fmt.Println("Error on create file:", err)
	}
	defer file.Close()

	className := helpers.Capitalize(name)

	_, err = file.WriteString(fmt.Sprintf("export class %s {}", className+template.name))

	if err != nil {
		fmt.Println("Error on write file:", err)
	}

	err = file.Sync()

	if err != nil {
		fmt.Println("Error on save file:", err)
	}

	fmt.Println("File created", filename)
}

type Template struct {
	name  string
	sufix string
}

func templates(fileType string) *Template {
	template := Template{}
	switch fileType {
	case "uc":
		template.sufix = "-usecase"
		template.name = "UseCase"
	}

	return &template
}
