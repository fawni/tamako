package cmd

import (
	"encoding/json"
	"fmt"
	"os"

	"github.com/fawni/tamako/api"
	"github.com/parnurzeal/gorequest"
	"github.com/spf13/cobra"
)

// rootCmd represents the base command when called without any subcommands
var (
	id    int64
	limit int

	rootCmd = &cobra.Command{
		Use:   "tamako",
		Short: "🐞 Cozy anonyomus whispers",

		CompletionOptions: cobra.CompletionOptions{
			DisableDefaultCmd: true,
		},
		RunE: func(cmd *cobra.Command, args []string) error {
			return run(args)
		},
	}
)

// Execute adds all child commands to the root command and sets flags appropriately.
// This is called by main.main(). It only needs to happen once to the rootCmd.
func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func run(args []string) error {
	req := gorequest.New()
	var url string
	if id != 0 {
		url = fmt.Sprintf("http://localhost:8714/api/whisper/%d?pretty=true", id)
	} else {
		url = "http://localhost:8714/api/whisper?pretty=true"
	}

	if limit != 0 {
		url = fmt.Sprintf("%s&limit=%d", url, limit)
	}

	_, body, errs := req.Get(url).End()
	if errs != nil {
		return errs[0]
	}

	if id != 0 || limit == 1 {
		var whisper api.Whisper
		if err := json.Unmarshal([]byte(body), &whisper); err != nil {
			return err
		}

		var name string
		if !(whisper.Name == "") {
			name = whisper.Name
		} else {
			name = "anon"
		}

		fmt.Printf("%s: %s\n", name, whisper.Message)
	} else {
		var whispers []api.Whisper
		if err := json.Unmarshal([]byte(body), &whispers); err != nil {
			return err
		}

		for _, whisper := range whispers {
			var name string
			if !(whisper.Name == "") {
				name = whisper.Name
			} else {
				name = "anon"
			}
			fmt.Printf("%s: %s\n", name, whisper.Message)
		}
	}

	return nil
}

func init() {
	rootCmd.Flags().Int64VarP(&id, "id", "i", 0, "Whisper Snowflake ID")
	rootCmd.Flags().IntVarP(&limit, "limit", "l", 0, "Limit the number of whispers to return")
}
