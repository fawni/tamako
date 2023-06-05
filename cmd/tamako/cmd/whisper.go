/*
Copyright © 2021 obscurity
Licensed under the Open Software License version 3.0
*/
package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

// whisperCmd represents the whisper command
var whisperCmd = &cobra.Command{
	Use:   "whisper",
	Short: "Whisper a message",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("whisper called")
	},
}

func init() {
	rootCmd.AddCommand(whisperCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// whisperCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// whisperCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
