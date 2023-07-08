package minimal

import (
	"fmt"

	"github.com/fawni/tamako/cmd/mochi/tamako"
	"github.com/fawni/tamako/cmd/mochi/tui/styles"
)

func Render(w tamako.Whisper) {
	res := titleMargin(styles.TitleStyle.Render("tamako") + fmt.Sprintf(" - %s", snowflakeStyle(fmt.Sprintf("%d", w.Snowflake))))
	res += line("\nName", name(w.Name))
	res += line("Message", w.Message)
	res += line("Private", fmt.Sprintf("%t", w.Private))
	res += line("Timestamp", w.Timestamp)

	fmt.Println(res)
}

func name(n string) string {
	if n == "anon" {
		return mutedStyle(n)
	}
	return n
}

func line(key string, value string) string {
	return textStyle(fmt.Sprintf("%s: %s", keyStyle(key), value)) + "\n"
}
