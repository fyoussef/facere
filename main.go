package main

import (
	"bufio"
	"fmt"
	"os"

	"golang.org/x/text/cases"
	"golang.org/x/text/language"
)

func main() {
	terminalReader := bufio.NewReader(os.Stdin)

	fmt.Print("Digite o nome do arquivo: ")
	line, _, err := terminalReader.ReadLine()

	if err != nil {
		fmt.Println("Error to read terminal:", err)
	}

	commands := string(line[:])

	file, err := os.Create(commands + ".ts")

	if err != nil {
		fmt.Println("Error on create file:", err)
	}
	defer file.Close()

	title := cases.Title(language.Und, cases.NoLower)
	filename := title.String(commands)

	_, err = file.WriteString(fmt.Sprintf("export class %s {}", filename))

	if err != nil {
		fmt.Println("Error on write file:", err)
	}

	err = file.Sync()

	if err != nil {
		fmt.Println("Error on save file:", err)
	}

	fmt.Println("File created", filename)
}
