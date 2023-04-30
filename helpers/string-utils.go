package helpers

import (
	"golang.org/x/text/cases"
	"golang.org/x/text/language"
)

func Capitalize(arg string) string {
	title := cases.Title(language.Und, cases.NoLower)

	return title.String(arg)
}
