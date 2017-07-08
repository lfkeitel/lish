package exec

import (
	"os"
	"os/exec"
)

func Run(name string, args []string, env map[string]string, cd string) error {
	cmd := exec.Command(name, args...)
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.Env = envMapToSlice(env)
	cmd.Dir = cd

	return cmd.Run()
}

func envMapToSlice(env map[string]string) []string {
	if env == nil {
		return nil
	}

	newenv := make([]string, len(env))

	i := 0
	for k, v := range env {
		newenv[i] = k + "=" + v
		i++
	}

	return newenv
}
