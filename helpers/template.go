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
		template.Type = "class"
	case "usecase":
		template.Sufix = ".usecase"
		template.Name = "UseCase"
		template.Type = "class"
	case "ent":
		template.Sufix = ".entity"
		template.Name = "Entity"
		template.Type = "class"
	case "entity":
		template.Sufix = ".entity"
		template.Name = "Entity"
		template.Type = "class"
	case "repo":
		template.Sufix = ".repository"
		template.Name = "Repository"
		template.Type = "class"
	case "repository":
		template.Sufix = ".repository"
		template.Name = "Repository"
		template.Type = "class"
	case "itf":
		template.Sufix = ".interface"
		template.Name = ""
		template.Type = "interface"
	case "interface":
		template.Sufix = ".interface"
		template.Name = ""
		template.Type = "interface"
	}

	return &template
}

func TemplateContent(templateType, templateName string) (string, error) {
	templateName = Capitalize(templateName)

	if templateType != "class" && templateType != "interface" {
		return "", errors.New("Unsupported template type")
	}

	if templateType == "class" {
		return fmt.Sprintf("export class %s {}", templateName), nil
	} else if templateType == "interface" {
		return fmt.Sprintf("export interface I%s {}", templateName), nil
	} else {
		return "", errors.New("Template not found")
	}

}
