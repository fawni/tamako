package styles

import lg "github.com/charmbracelet/lipgloss"

const (
	Red          = "#E7363B"
	Green        = "#4CC767"
	Gainsboro    = "#D9D8DC"
	Gray         = "#928E92"
	White        = "#FFFFFF"
	Black        = "#120c0e"
	Salmon       = "#EB99A1"
	DimmedSalmon = "#E26F7A"
)

var (
	Colors = struct {
		Primary    lg.TerminalColor
		Secondary  lg.TerminalColor
		Background lg.TerminalColor
		Text       lg.TerminalColor
		White      lg.TerminalColor
		Muted      lg.TerminalColor
		Red        lg.TerminalColor
		Green      lg.TerminalColor
	}{
		Primary:    lg.Color(Salmon),
		Secondary:  lg.Color(DimmedSalmon),
		Background: lg.Color(Black),
		Text:       lg.Color(Gainsboro),
		Muted:      lg.Color(Gray),
		White:      lg.Color(White),
		Red:        lg.Color(Red),
		Green:      lg.Color(Green),
	}
)
