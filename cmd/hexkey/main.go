package main

import (
	"fmt"
	"os"

	"github.com/lfkeitel/lish/env"
	"github.com/lfkeitel/lish/shell"
	"github.com/lfkeitel/lish/terminal"
)

func main() {
	term, err := terminal.New()
	if err != nil {
		panic(err)
	}
	defer term.Close()
	term.SetHexDebug()

	s := shell.New(env.New(), term)
	s.NoExec()

	if err := s.Run(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
