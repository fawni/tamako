package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

var whisperCmd = &cobra.Command{
	Use:   "whisper",
	Short: "Whisper a message",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("whisper called")
	},
}

func init() {
	rootCmd.AddCommand(whisperCmd)
}
