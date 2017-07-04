package main

import (
	"bytes"
	"flag"
	"fmt"
	"os"

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

	term, err := terminal.New(os.Stdin.Fd())
	if err != nil {
		panic(err)
	}
	defer term.Close()
	term.SetPrompt(">> ")
	if err := term.SetRawMode(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	if printHex {
		term.SetHexDebug()
	}

	for {
		line, err := term.ReadLine()
		if err != nil {
			fmt.Println(err)
			os.Exit(1)
		}

		if bytes.Equal(line, []byte("exit")) {
			break
		}

		term.Println(string(line))
	}
}
