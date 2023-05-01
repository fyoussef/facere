package helpers

type Template struct {
	Name  string
	Sufix string
}

func Templates(fileType string) *Template {
	template := Template{}
	switch fileType {
	case "uc":
		template.Sufix = "-usecase"
		template.Name = "UseCase"
	}

	return &template
}
