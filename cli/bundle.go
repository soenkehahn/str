package cli

import (
	"github.com/evanw/esbuild/pkg/api"
)

func bundle(inputCode string, outputFile string) error {
	buildResult := api.Build(api.BuildOptions{
		EntryPoints: []string{},
		Outfile:     outputFile,
		Bundle:      true,
		Write:       true,
		Platform:    api.PlatformNode,
		Stdin: &api.StdinOptions{
			Contents:   inputCode,
			ResolveDir: ".",
			Sourcefile: "<str test runner>",
		},
		Plugins: []api.Plugin{injectDirname},
	})
	if len(buildResult.Errors) > 0 {
		formattedErrors := api.FormatMessages(buildResult.Errors, api.FormatMessagesOptions{
			Color: true,
		})
		for _, error := range formattedErrors {
			print(error)
		}
		return &BundleError{}
	}
	return nil
}

type BundleError struct{}

func (e *BundleError) Error() string {
	return "bundle error"
}
