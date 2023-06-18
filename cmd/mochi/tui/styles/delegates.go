package styles

import (
	"github.com/charmbracelet/bubbles/list"
)

func NewListDelegate() list.DefaultDelegate {
	d := list.NewDefaultDelegate()
	d.Styles.SelectedTitle = d.Styles.SelectedTitle.Foreground(Colors.Primary).BorderLeftForeground(Colors.Primary)
	d.Styles.SelectedDesc = d.Styles.SelectedDesc.Foreground(Colors.Secondary).BorderLeftForeground(Colors.Primary)
	d.Styles.NormalTitle = d.Styles.NormalTitle.Foreground(Colors.Text)

	return d
}
