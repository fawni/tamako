package keys

import "github.com/charmbracelet/bubbles/key"

type Keymap struct {
	Copy    key.Binding
	Refresh key.Binding
	Delete  key.Binding
}

func NewKeymap() *Keymap {
	return &Keymap{
		Copy: key.NewBinding(
			key.WithKeys("enter"),
			key.WithHelp("enter", "copy"),
		),
		Refresh: key.NewBinding(
			key.WithKeys("r"),
			key.WithHelp("r", "refresh"),
		),
		Delete: key.NewBinding(
			key.WithKeys("delete"),
			key.WithHelp("delete", "delete"),
			key.WithDisabled(), // TODO: implement auth
		),
	}
}
