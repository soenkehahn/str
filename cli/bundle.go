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
		Plugins: []api.Plugin{injectDirname, nonRelativeImportsAreExternal},
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

var nonRelativeImportsAreExternal api.Plugin = api.Plugin{
	Name: "make non relative imports (e.g. from node_modules) external",
	Setup: func(build api.PluginBuild) {
		build.OnResolve(api.OnResolveOptions{Filter: "^[^\\.]"},
			func(args api.OnResolveArgs) (api.OnResolveResult, error) {
				return api.OnResolveResult{External: true}, nil
			})
	},
}
