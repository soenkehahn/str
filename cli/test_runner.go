package cli

import (
	"fmt"
	"os"
	"os/exec"
	"syscall"

	"github.com/lithammer/dedent"
)

func runnerCode(testFiles []string) string {
	code := `
		import { _strTestRunner, describe } from "str";
		async function main() {`
	for _, testFile := range testFiles {
		code += fmt.Sprintf(`
			await _strTestRunner.enterTestFile("%s", () => import("./%s"));`,
			testFile, testFile)
	}
	code += `
			await _strTestRunner.runTests();
		}
		main();
	`
	return dedent.Dedent(code)
}

type runner struct {
	failed bool
}

func Run(testFiles []string) (int, error) {
	runner := runner{}
	err := runner.runTestFiles(testFiles)
	if err != nil {
		return 1, err
	}
	if runner.failed {
		return 1, nil
	} else {
		return 0, nil
	}
}

func (runner *runner) runTestFiles(testFiles []string) error {
	strDistDir, err := os.MkdirTemp("", "str-bundle")
	if err != nil {
		return err
	}
	defer os.RemoveAll(strDistDir)
	os.Mkdir(strDistDir, 0755)
	bundleFile := strDistDir + "/main.js"
	err = bundle(runnerCode(testFiles), bundleFile)
	if err != nil {
		return err
	}
	return runner.runBundle(bundleFile)
}

func writeFile(file string, content string) error {
	runnerFile, err := os.Create(file)
	if err != nil {
		return err
	}
	_, err = runnerFile.WriteString(content)
	if err != nil {
		return err
	}
	runnerFile.Close()
	return nil
}

func (runner *runner) runBundle(bundleFile string) error {
	command := exec.Command("node", bundleFile)
	workingDirectory, err := os.Getwd()
	if err != nil {
		return err
	}
	command.Env = append(
		os.Environ(),
		fmt.Sprintf("NODE_PATH=%s/node_modules", workingDirectory),
	)
	command.Stdout = os.Stdout
	command.Stderr = os.Stderr
	err = command.Run()
	if err != nil {
		if exitErr, ok := err.(*exec.ExitError); ok {
			if status, ok := exitErr.Sys().(syscall.WaitStatus); ok {
				if status.ExitStatus() != 0 {
					runner.failed = true
				}
				return nil
			} else {
				return exitErr
			}
		} else {
			return exitErr
		}
	}
	return nil
}
