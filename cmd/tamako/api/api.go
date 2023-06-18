package api

import (
	"encoding/json"
	"io"
	"net/http"
)

type Whisper struct {
	Name      string `json:"name,omitempty"`
	Message   string `json:"message,omitempty"`
	Private   bool   `json:"private,omitempty"`
	Snowflake int64  `json:"snowflake,omitempty"`
	Timestamp string `json:"timestamp,omitempty"`
}

func List() ([]Whisper, error) {
	resp, err := http.Get("http://localhost:8714/api/whisper")
	if err != nil {
		return []Whisper{}, err
	}

	defer resp.Body.Close()
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return []Whisper{}, err
	}
	var whispers []Whisper
	if err := json.Unmarshal(body, &whispers); err != nil {
		return []Whisper{}, err
	}
	return whispers, nil
}
