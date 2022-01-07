package main

import (
	"os"

	"github.com/soenkehahn/str/cli"
)

func main() {
	testFile := os.Args[1]
	exitCode, err := cli.RunTestFile(testFile)
	if err != nil {
		if _, ok := err.(*cli.BundleError); ok {
			os.Exit(1)
		} else {
			panic(err)
		}
	}
	os.Exit(exitCode)
}
