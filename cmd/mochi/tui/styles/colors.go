package styles

import lg "github.com/charmbracelet/lipgloss"

const (
	red          = "1"
	green        = "2"
	white        = "#FFFFFF"
	gainsboro    = "#D9D8DC"
	black        = "#120c0e"
	salmon       = "#EB99A1"
	dimmedSalmon = "#E26F7A"
)

var (
	Colors = struct {
		Primary    lg.TerminalColor
		Secondary  lg.TerminalColor
		Background lg.TerminalColor
		Text       lg.TerminalColor
		White      lg.TerminalColor
		Red        lg.TerminalColor
		Green      lg.TerminalColor
	}{
		Primary:    lg.Color(salmon),
		Secondary:  lg.Color(dimmedSalmon),
		Background: lg.Color(black),
		Text:       lg.Color(gainsboro),
		White:      lg.Color(white),
		Red:        lg.Color(red),
		Green:      lg.Color(green),
	}
)
