package minimal

import (
	"fmt"

	"github.com/fawni/mochi/tamako"
	"github.com/fawni/mochi/tui/styles"
)

func Render(w tamako.Whisper) {
	res := titleMargin(styles.TitleStyle.Render("tamako") + fmt.Sprintf(" - %s", mutedStyle(fmt.Sprintf("%d", w.Snowflake))))
	res += textStyle(fmt.Sprintf("\n%s: %s", keyStyle("Name"), Name(w.Name)))
	res += "\n"
	res += textStyle(fmt.Sprintf("%s: %s", keyStyle("Message"), w.Message))
	res += "\n"
	res += textStyle(fmt.Sprintf("%s: %t", keyStyle("Private"), w.Private))
	res += "\n"
	res += textStyle(fmt.Sprintf("%s: %s", keyStyle("Timestamp"), w.Timestamp))
	res += "\n"

	fmt.Println(res)
}

func Name(n string) string {
	if n == "anon" {
		return mutedStyle(n)
	}
	return n
}
