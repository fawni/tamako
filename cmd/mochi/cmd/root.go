package cmd

import (
	"fmt"
	"os"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/fawni/mochi/tamako"
	"github.com/fawni/mochi/tui"
	"github.com/fawni/mochi/tui/minimal"
	"github.com/fawni/mochi/tui/styles"
	"github.com/muesli/termenv"
	"github.com/spf13/cobra"
)

var (
	id    int64
	limit int

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

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(styles.Error(err.Error()))
		os.Exit(1)
	}
}

func run(args []string) error {
	if id != 0 {
		whisper, err := tamako.Get(id)
		if err != nil {
			return err
		}

		minimal.Render(whisper)
	} else {
		output := termenv.NewOutput(os.Stdout)
		defer output.Reset()
		output.SetBackgroundColor(output.Color(styles.Black))

		whispers, err := tamako.List(limit)
		if err != nil {
			return err
		}

		if _, err := tea.NewProgram(tui.New(whispers), tea.WithAltScreen()).Run(); err != nil {
			return err
		}
	}

	return nil
}

func init() {
	rootCmd.Flags().Int64VarP(&id, "id", "i", 0, "Whisper Snowflake ID")
	rootCmd.Flags().IntVarP(&limit, "limit", "l", 0, "Limit the number of whispers to return")
}
