package main

import (
	"flag"
	"fmt"
	"os"

	"github.com/lfkeitel/lish/env"
	"github.com/lfkeitel/lish/shell"
	"github.com/lfkeitel/lish/terminal"
)

var (
	printHex bool
)

func init() {
	flag.BoolVar(&printHex, "hexdebug", false, "Print the hex values emitted by a key press")
}

func main() {
	flag.Parse()
	fmt.Println("Welcome to Lish")
	fmt.Println("Type a command to begin")

	term, err := terminal.New()
	if err != nil {
		panic(err)
	}
	defer term.Close()
	if printHex {
		term.SetHexDebug()
	}

	s := shell.New(env.FromSlice(os.Environ()), term)

	if err := s.Run(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
