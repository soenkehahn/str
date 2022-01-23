package cli

import (
	"fmt"
	"os"
	"os/exec"
	"syscall"

	"github.com/lithammer/dedent"
)

func runnerCode(testFile string) string {
	return dedent.Dedent(fmt.Sprintf(`
		import { _strTestRunner } from "str";
		async function main() {
			_strTestRunner.testFile = "%s";
			await import("./%s");
			await _strTestRunner.runTests();
		}
		main();
	`, testFile, testFile))
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
	for _, testFile := range testFiles {
		err := runner.runTestFile(testFile)
		if err != nil {
			return err
		}
	}
	return nil
}

func (runner *runner) runTestFile(testFile string) error {
	strDistDir, err := os.MkdirTemp("", "str-bundle")
	if err != nil {
		return err
	}
	defer os.RemoveAll(strDistDir)
	os.Mkdir(strDistDir, 0755)
	bundleFile := strDistDir + "/main.js"
	err = bundle(runnerCode(testFile), bundleFile)
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
	command.Stdout = os.Stdout
	command.Stderr = os.Stderr
	err := command.Run()
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
