package main

import "github.com/fyoussef/scafolding.git/cmd"

// import (
// 	"bufio"
// 	"fmt"
// 	"log"
// 	"os"
// 	"strings"

// 	"github.com/fyoussef/scafolding.git/helpers"
// )

// func main() {
// 	terminalReader := bufio.NewReader(os.Stdin)

// 	fmt.Print("Digite o nome do arquivo: ")
// 	line, _, err := terminalReader.ReadLine()

// 	if err != nil {
// 		fmt.Println("Error to read terminal:", err)
// 	}

// 	commands := string(line[:])

// 	args := strings.Fields(commands)

// 	path := args[2]

// 	if path == "" {
// 		log.Fatal("Filename has missing")
// 	}

// 	template := helpers.Templates(args[1])

// 	helpers.CreateFile(path, template)

// }

func main() {
	cmd.Execute()
}
