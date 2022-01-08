package cli

import (
	"fmt"
	"io/ioutil"
	"log"
	"path/filepath"

	"github.com/evanw/esbuild/pkg/api"
)

var injectDirname api.Plugin = api.Plugin{
	Name: "inject __dirname",
	Setup: func(build api.PluginBuild) {
		options := api.OnLoadOptions{Filter: "\\.(js|jsx|ts|tsx)$"}
		build.OnLoad(options, onLoad)
	},
}

func onLoad(args api.OnLoadArgs) (api.OnLoadResult, error) {
	__dirname := filepath.Dir(args.Path)
	code, err := ioutil.ReadFile(args.Path)
	if err != nil {
		return api.OnLoadResult{}, err
	}
	contents := fmt.Sprintf("__dirname = '%s';", __dirname) + string(code)
	result := api.OnLoadResult{
		Contents: &contents,
		Loader:   pickLoader(args),
	}
	return result, nil
}

func pickLoader(args api.OnLoadArgs) api.Loader {
	loader, ok := loaders[filepath.Ext(args.Path)]
	if !ok {
		log.Fatalf("impossible extension: %s", filepath.Ext(args.Path))
	}
	return loader
}

var loaders = map[string]api.Loader{
	".js":  api.LoaderJS,
	".jsx": api.LoaderJSX,
	".ts":  api.LoaderTS,
	".tsx": api.LoaderTSX,
}
