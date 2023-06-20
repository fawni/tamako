package tamako

import (
	"encoding/json"
	"fmt"

	"github.com/parnurzeal/gorequest"
)

const (
	BASE_URL = "https://tamako.pii.at"
	BASE_API = BASE_URL + "/api/whisper"
)

type Whisper struct {
	Name      string `json:"name,omitempty"`
	Message   string `json:"message,omitempty"`
	Private   bool   `json:"private,omitempty"`
	Snowflake int64  `json:"snowflake,omitempty"`
	Timestamp string `json:"timestamp,omitempty"`
}

func (w Whisper) Title() string       { return w.Message }
func (w Whisper) Description() string { return fmt.Sprintf("%s • %s", w.Name, w.Timestamp) }
func (w Whisper) FilterValue() string { return fmt.Sprintf("%s %s %d", w.Message, w.Name, w.Snowflake) }

func Get(id int64) (Whisper, error) {
	req := gorequest.New()
	var whisper Whisper

	_, body, errs := req.Get(fmt.Sprintf("%s/%d?pretty=true", BASE_API, id)).End()
	if errs != nil {
		return Whisper{}, errs[0]
	}

	if err := json.Unmarshal([]byte(body), &whisper); err != nil {
		return Whisper{}, err
	}

	if whisper.Name == "" {
		whisper.Name = "anon"
	}

	return whisper, nil
}

func List(id int64, limit int) ([]Whisper, error) {
	req := gorequest.New()
	var whispers []Whisper

	var url string
	if id != 0 {
		url = fmt.Sprintf("%s/%d?pretty=true", BASE_API, id)
	} else {
		url = fmt.Sprintf("%s?pretty=true", BASE_API)
	}
	if limit != 0 {
		url = fmt.Sprintf("%s&limit=%d", url, limit)
	}

	_, body, errs := req.Get(url).End()
	if errs != nil {
		return []Whisper{}, errs[0]
	}

	if id != 0 {
		var whisper Whisper
		if err := json.Unmarshal([]byte(body), &whisper); err != nil {
			return []Whisper{}, err
		}

		if whisper.Name == "" {
			whisper.Name = "anon"
		}

		return []Whisper{whisper}, nil
	}

	if err := json.Unmarshal([]byte(body), &whispers); err != nil {
		return []Whisper{}, err
	}

	for i, whisper := range whispers {
		if whisper.Name == "" {
			whispers[i].Name = "anon"
		}
	}

	return whispers, nil
}
