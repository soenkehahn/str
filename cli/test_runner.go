package cli

import (
	"fmt"
	"os"
	"os/exec"
	"syscall"

	"github.com/lithammer/dedent"
)

func RunTestFile(testFile string) (int, error) {
	strDistDir, err := os.MkdirTemp("", "str-bundle")
	if err != nil {
		return 1, err
	}
	defer os.RemoveAll(strDistDir)
	os.Mkdir(strDistDir, 0755)
	bundleFile := strDistDir + "/main.js"
	err = bundle(runnerCode(testFile), bundleFile)
	if err != nil {
		return 1, err
	}
	return runBundle(bundleFile)
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

func runnerCode(testFile string) string {
	return dedent.Dedent(fmt.Sprintf(`
		import { _strTestRunner } from "str";
		async function main() {
			_strTestRunner.setTestFile("%s");
			await import("./%s");
			_strTestRunner.finalize();
		}
		main();
	`, testFile, testFile))
}

func runBundle(bundleFile string) (int, error) {
	command := exec.Command("node", bundleFile)
	command.Stdout = os.Stdout
	command.Stderr = os.Stderr
	err := command.Run()
	if err != nil {
		if exitErr, ok := err.(*exec.ExitError); ok {
			if status, ok := exitErr.Sys().(syscall.WaitStatus); ok {
				return status.ExitStatus(), nil
			} else {
				return 1, exitErr
			}
		} else {
			return 1, exitErr
		}
	}
	return 0, nil
}
