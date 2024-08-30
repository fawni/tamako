package styles

import lg "github.com/charmbracelet/lipgloss"

var (
	AppStyle = lg.NewStyle().
			Margin(1, 2)
	TitleStyle = lg.NewStyle().
			Foreground(Colors.White).
			Background(Colors.Primary).
			Bold(true).
			Padding(0, 1)

	Primary = lg.NewStyle().
		Foreground(Colors.Primary).
		Render
	Error = lg.NewStyle().
		Foreground(Colors.Red).
		Render
	Success = lg.NewStyle().
		Foreground(Colors.Green).
		Render
)
