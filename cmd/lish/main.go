package main

import (
	"bytes"
	"fmt"
	"os"

	"github.com/lfkeitel/lish/terminal"
)

func main() {
	fmt.Println("Welcome to Lish")
	fmt.Println("Type a command to begin")

	term := terminal.NewTerminal(os.Stdin.Fd())
	defer term.Close()
	term.SetPrompt(">> ")
	if err := term.SetRawMode(); err != nil {
		fmt.Println(err)
		os.Exit(1)
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
