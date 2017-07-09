package shell

import (
	"os"

	"github.com/lfkeitel/lish/env"
	"github.com/lfkeitel/lish/exec"
)

const (
	defaultPrompt = "$ "
)

type Terminal interface {
	EnableRawMode() error
	DisableRawMode() error
	ReadLine(prompt string) (string, error)
	ReadPassword(prompt string) (string, error)
	Println(a ...interface{}) (int, error)
	AddHistory(item string)
}

type Shell struct {
	env      *env.Environment
	terminal Terminal
	pwd      string
	exec     bool
}

func New(environ *env.Environment, term Terminal) *Shell {
	if environ == nil {
		environ = env.New()
	}

	pwd, err := os.Getwd()
	if err != nil {
		panic("Can't get current directory")
	}

	return &Shell{
		env:      environ,
		terminal: term,
		pwd:      pwd,
		exec:     true,
	}
}

func (s *Shell) NoExec() {
	s.exec = false
}

func (s *Shell) Run() error {
	defer func() {
		if err := recover(); err != nil {
			s.terminal.DisableRawMode()
			s.terminal.Println(err)
		}
	}()

	if err := s.terminal.EnableRawMode(); err != nil {
		return err
	}

	for {
		line, err := s.terminal.ReadLine(s.env.GetDefault("LISH_PROMPT", defaultPrompt))
		if err != nil {
			return err
		}

		args := ParseShellArgs(line)
		if len(args) == 0 {
			continue
		}

		s.terminal.AddHistory(line)

		if args[0] == "exit" {
			break
		}

		if !s.exec {
			continue
		}

		if f, exists := builtins[args[0]]; exists {
			if err := f(s, args[1:]); err != nil {
				s.Println(err.Error())
			}
			continue
		}

		s.terminal.DisableRawMode()
		err = exec.Run(args[0], args[1:], s.env, s.pwd)
		s.terminal.EnableRawMode()

		if err != nil {
			s.Println(err.Error())
		}
	}

	return nil
}

func (s *Shell) Println(a ...interface{}) {
	s.terminal.Println(a...)
}

func (s *Shell) GetEnv() *env.Environment {
	return s.env
}
