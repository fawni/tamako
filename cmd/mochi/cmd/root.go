package cmd

import (
	"fmt"
	"os"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/fawni/tamako/cmd/mochi/tamako"
	"github.com/fawni/tamako/cmd/mochi/tui"
	"github.com/fawni/tamako/cmd/mochi/tui/minimal"
	"github.com/fawni/tamako/cmd/mochi/tui/styles"
	"github.com/muesli/termenv"
	"github.com/spf13/cobra"
)

var (
	id    int64
	limit int
	url   string

	rootCmd = &cobra.Command{
		Use:   "mochi",
		Short: "🐞 Cozy anonyomus whispers",
		CompletionOptions: cobra.CompletionOptions{
			DisableDefaultCmd: true,
		},
		RunE: func(cmd *cobra.Command, args []string) error {
			return run(args)
		},
	}
)

func init() {
	rootCmd.Flags().Int64VarP(&id, "id", "i", 0, "Whisper Snowflake ID")
	rootCmd.Flags().IntVarP(&limit, "limit", "l", 0, "Limit the number of whispers to return")
	rootCmd.Flags().StringVarP(&url, "url", "u", "https://tamako.pii.at", "Base URL of tamako")
}

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(styles.Error(err.Error()))
		os.Exit(1)
	}
}

func run(args []string) error {
	if id != 0 {
		whisper, err := tamako.Get(url, id)
		if err != nil {
			return err
		}

		minimal.Render(whisper)
	} else {
		output := termenv.NewOutput(os.Stdout)
		defer output.Reset()
		output.SetBackgroundColor(output.Color(styles.Black))

		whispers, err := tamako.List(url, limit)
		if err != nil {
			return err
		}

		if _, err := tea.NewProgram(tui.New(url, whispers)).Run(); err != nil {
			return err
		}
	}

	return nil
}
