package minimal

import (
	"codeberg.org/fawn/tamako/cmd/mochi/tui/styles"
	lg "github.com/charmbracelet/lipgloss"
)

var (
	titleMargin    = lg.NewStyle().Margin(1, 0, 1, 2).Render
	textStyle      = lg.NewStyle().MarginLeft(2).Render
	keyStyle       = lg.NewStyle().Foreground(styles.Colors.Primary).Bold(true).Render
	mutedStyle     = lg.NewStyle().Foreground(styles.Colors.Muted).Italic(true).Render
	snowflakeStyle = lg.NewStyle().Italic(true).Render
)
