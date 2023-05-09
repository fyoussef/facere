package helpers

import (
	"errors"
	"fmt"
)

type Template struct {
	Name  string
	Sufix string
	Type  string
}

func Templates(fileType string) *Template {
	template := Template{}
	switch fileType {
	case "uc":
		template.Sufix = ".usecase"
		template.Name = "UseCase"
	case "usecase":
		template.Sufix = ".usecase"
		template.Name = "UseCase"
	case "ent":
		template.Sufix = ".entity"
		template.Name = "Entity"
	case "entity":
		template.Sufix = ".entity"
		template.Name = "Entity"
	case "repo":
		template.Sufix = ".repository"
		template.Name = "Repository"
	case "repository":
		template.Sufix = ".repository"
		template.Name = "Repository"
	}

	return &template
}

func TemplateContent(templateType, templateName string) (string, error) {
	templateName = Capitalize(templateName)

	if templateType == "class" {
		return fmt.Sprintf("export class %s {}", templateName), nil
	} else if templateType == "interface" {
		return fmt.Sprintf("export interface I%s {}", templateName), nil
	}

	return "", errors.New("Unsupported template type")
}
