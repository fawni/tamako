package tui

import (
	"fmt"

	"github.com/atotto/clipboard"
	"github.com/charmbracelet/bubbles/key"
	"github.com/charmbracelet/bubbles/list"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/fawni/tamako/cmd/mochi/tamako"
	"github.com/fawni/tamako/cmd/mochi/tui/keys"
	"github.com/fawni/tamako/cmd/mochi/tui/styles"
)

type TUI struct {
	url  string
	list list.Model
	keys *keys.Keymap
}

func (TUI) Init() tea.Cmd {
	return tea.EnterAltScreen
}

func (t TUI) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch {
		case key.Matches(msg, t.keys.Copy):
			if t.list.SelectedItem() == nil {
				break
			}

			whisper := t.list.SelectedItem().(tamako.Whisper)
			if err := clipboard.WriteAll(fmt.Sprintf("%d", whisper.Snowflake)); err != nil {
				return t, tea.Cmd(t.list.NewStatusMessage(styles.Error(err.Error())))
			}

			return t, tea.Cmd(t.list.NewStatusMessage(styles.Success(fmt.Sprintf("Copied whisper id %d", whisper.Snowflake))))

		case key.Matches(msg, t.keys.Refresh):
			whispers, err := tamako.List(t.url, 0)
			if err != nil {
				return t, tea.Cmd(t.list.NewStatusMessage(t.url))
			}

			items := make([]list.Item, 0, len(whispers))
			for _, whisper := range whispers {
				items = append(items, whisper)
			}

			return t, tea.Batch(t.list.SetItems(items),
				t.list.NewStatusMessage(styles.Success("Refreshed")))
		}

	case tea.WindowSizeMsg:
		h, v := styles.AppStyle.GetFrameSize()
		t.list.SetSize(msg.Width-h, msg.Height-v)
	}

	var cmd tea.Cmd
	t.list, cmd = t.list.Update(msg)

	return t, tea.Cmd(cmd)
}

func (t TUI) View() string {
	return styles.AppStyle.Render(t.list.View())
}

func New(url string, whispers []tamako.Whisper) TUI {
	items := make([]list.Item, 0, len(whispers))
	for _, whisper := range whispers {
		items = append(items, whisper)
	}

	delegate := styles.NewListDelegate()
	tui := TUI{url: url, list: list.New(items, delegate, 0, 0), keys: keys.NewKeymap()}
	tui.list.Title = "tamako"
	tui.list.Styles.Title = styles.TitleStyle
	tui.list.Styles.FilterPrompt = tui.list.Styles.FilterPrompt.Foreground(styles.Colors.Primary)
	tui.list.Styles.FilterCursor = tui.list.Styles.FilterCursor.Foreground(styles.Colors.Primary)

	tui.list.AdditionalShortHelpKeys = func() []key.Binding { return []key.Binding{tui.keys.Copy, tui.keys.Refresh} }
	tui.list.AdditionalFullHelpKeys = func() []key.Binding { return []key.Binding{tui.keys.Copy, tui.keys.Refresh, tui.keys.Delete} }

	return tui
}
