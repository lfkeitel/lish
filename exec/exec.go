package exec

import (
	"os"
	"os/exec"

	"github.com/lfkeitel/lish/env"
)

func Run(name string, args []string, env *env.Environment, cd string) error {
	cmd := exec.Command(name, args...)
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.Env = env.ToSlice()
	cmd.Dir = cd

	return cmd.Run()
}
