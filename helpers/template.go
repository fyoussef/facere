package helpers

import (
	"errors"
	"fmt"
)

type Template struct {
	Name  string
	Sufix string
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
	case "itf":
		template.Sufix = ".interface"
		template.Name = ""
	case "interface":
		template.Sufix = ".interface"
		template.Name = ""
	}

	return &template
}

func TemplateContent(templateType, templateName string) (string, error) {
	templateName = Capitalize(templateName)

	classTemplates := []string{"uc", "usecase", "ent", "entity"}
	itfTemplates := []string{"itf", "interface"}

	isClassTemplace := Contains(classTemplates, templateType)
	isItfTemplate := Contains(itfTemplates, templateType)

	if !isClassTemplace && !isItfTemplate {
		return "", errors.New("Unsupported template type")
	}

	if isClassTemplace {
		return fmt.Sprintf("export class %s {}", templateName), nil
	} else if isItfTemplate {
		return fmt.Sprintf("export interface I%s {}", templateName), nil
	} else {
		return "", errors.New("Template not found")
	}

}
