package keys

import "github.com/charmbracelet/bubbles/key"

type Keymap struct {
	Enter  key.Binding
	Delete key.Binding
}

func NewKeymap() *Keymap {
	return &Keymap{
		Enter: key.NewBinding(
			key.WithKeys("enter"),
			key.WithHelp("enter", "copy"),
		),
		Delete: key.NewBinding(
			key.WithKeys("delete"),
			key.WithHelp("delete", "delete"),
			key.WithDisabled(), // TODO: implement auth
		),
	}
}
