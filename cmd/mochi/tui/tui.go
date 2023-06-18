package tui

import (
	"fmt"

	"github.com/atotto/clipboard"
	"github.com/charmbracelet/bubbles/key"
	"github.com/charmbracelet/bubbles/list"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/fawni/mochi/tamako"
	"github.com/fawni/mochi/tui/keys"
	"github.com/fawni/mochi/tui/styles"
)

type TUI struct {
	list list.Model
	keys *keys.Keymap
}

func (t TUI) Init() tea.Cmd {
	return nil
}

func (t TUI) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch {
		case key.Matches(msg, t.keys.Enter):
			if t.list.SelectedItem() == nil {
				break
			}
			whisper := t.list.SelectedItem().(tamako.Whisper)
			if err := clipboard.WriteAll(fmt.Sprintf("%d", whisper.Snowflake)); err != nil {
				t.list.NewStatusMessage(styles.Error(err.Error()))
			}
			t.list.NewStatusMessage(styles.Success(fmt.Sprintf("copied whisper id: %d", whisper.Snowflake)))
		}
	case tea.WindowSizeMsg:
		h, v := styles.AppStyle.GetFrameSize()
		t.list.SetSize(msg.Width-h, msg.Height-v)
	}

	var cmd tea.Cmd
	t.list, cmd = t.list.Update(msg)
	return t, cmd
}

func (t TUI) View() string {
	return styles.AppStyle.Render(t.list.View())
}

func New(whispers []tamako.Whisper) TUI {
	items := make([]list.Item, 0, len(whispers))
	for _, whisper := range whispers {
		items = append(items, whisper)
	}

	d := styles.NewListDelegate()
	t := TUI{list: list.New(items, d, 0, 0), keys: keys.NewKeymap()}
	t.list.Title = "tamako"
	t.list.Styles.Title = styles.TitleStyle

	t.list.AdditionalShortHelpKeys = func() []key.Binding {
		return []key.Binding{t.keys.Enter}
	}
	t.list.AdditionalFullHelpKeys = func() []key.Binding {
		return []key.Binding{t.keys.Enter, t.keys.Delete}
	}

	return t
}
