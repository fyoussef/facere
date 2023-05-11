package cmd

import (
	"fmt"
	"log"

	"github.com/fyoussef/scafolding.git/colors"
	"github.com/spf13/cobra"
)

var facereCmd = &cobra.Command{
	Use:   "facere",
	Short: "Facere is a simple scaffolding project to create files based in clean arch",
	Long: fmt.Sprintf(`
	Available templates to create
	%s
	| usecase        | us   | 
	|                |      |
	| entity         | ent  |
	|                |      |
	| repository     | repo |
	|                |      |
	| interface      | itf  |
	%s
`, colors.Blue, colors.Reset),
	Example: "  facere [usecase] [usecase/filename]",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("cmd", cmd)
		fmt.Println("args", args)
	},
}

func Execute() {
	if err := facereCmd.Execute(); err != nil {
		log.Fatal("Erro to execute facere", err)
	}
}
