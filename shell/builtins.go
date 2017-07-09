package shell

import (
	"errors"
	"os"
	"path/filepath"
)

type builtin func(s *Shell, args []string) error

var builtins map[string]builtin

func init() {
	builtins = make(map[string]builtin)

	builtins["cd"] = builtinCD
	builtins["pwd"] = builtinPWD
	builtins["def"] = builtinDefine
}

func builtinCD(s *Shell, args []string) error {
	path := ""
	if len(args) == 0 {
		path = s.GetEnv().Get("HOME")
	} else {
		path = args[0]
	}

	path, _ = filepath.Abs(path)
	if !isDir(path) {
		return errors.New("Path " + path + " doesn't exist or isn't a directory")
	}
	s.pwd = path
	os.Chdir(path)
	return nil
}

func builtinPWD(s *Shell, args []string) error {
	s.Println(s.pwd)
	return nil
}

func builtinDefine(s *Shell, args []string) error {
	if len(args) != 2 {
		return errors.New("def requires two arguments: def key value")
	}
	s.GetEnv().Set(args[0], args[1])
	return nil
}
