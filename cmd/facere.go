package cmd

import (
	"fmt"
	"log"
	"os"

	"github.com/fyoussef/scafolding.git/colors"
	"github.com/fyoussef/scafolding.git/helpers"
	"github.com/spf13/cobra"
)

var facereCmd = &cobra.Command{
	Use:   "facere [template] [dir/file | file]",
	Short: "Facere is a simple scaffolding project to create files based in clean arch",
	Long: fmt.Sprintf(`
Available templates to create:
	%s
	| usecase        | us   | 
	|                |      |
	| entity         | ent  |
	|                |      |
	| repository     | repo |
	|                |      |
	| interface      | itf  |
	%s
Attention: The /src directory will be created automaticly in first command and all files and folders will be created inside /src directory
`, colors.Blue, colors.Reset),
	Example: "  facere [usecase] [usecase/filename]",
	Run: func(cmd *cobra.Command, args []string) {

		if len(args) < 2 {
			log.Println("See available commands using facere -h")
			return
		}

		template := helpers.Templates(args[0])
		path := args[1]

		helpers.CreateFile(path, template)

	},
}

func Execute() {
	if err := facereCmd.Execute(); err != nil {
		log.Fatal("Erro to execute facere", err)
		os.Exit(1)
	}
}

func init() {
	facereCmd.Execute()
}
