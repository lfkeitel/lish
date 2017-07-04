package terminal

import (
	"errors"
	"fmt"
	"os"

	term "golang.org/x/crypto/ssh/terminal"
)

type Terminal struct {
	prompt   []byte
	term     *os.File
	echo     bool
	printHex bool

	oldstate *term.State
}

func New(fd uintptr) (*Terminal, error) {
	if !term.IsTerminal(int(fd)) {
		return nil, errors.New("file descriptor is not a valid terminal")
	}

	return &Terminal{
		term: os.NewFile(fd, "terminal"),
		echo: true,
	}, nil
}

func (t *Terminal) SetHexDebug() {
	t.printHex = true
}

func (t *Terminal) SetRawMode() error {
	var err error
	t.oldstate, err = term.MakeRaw(int(t.term.Fd()))
	return err
}

func (t *Terminal) Close() error {
	if t.oldstate != nil {
		return term.Restore(int(t.term.Fd()), t.oldstate)
	}
	return nil
}

func (t *Terminal) SetPrompt(prompt string) {
	t.prompt = []byte(prompt)
}

func (t *Terminal) ReadLine() ([]byte, error) {
	return t.read()
}

func (t *Terminal) ReadPassword() ([]byte, error) {
	t.echo = false
	line, err := t.read()
	t.echo = true
	return line, err
}

func (t *Terminal) read() ([]byte, error) {
	line := make([]byte, 1024)
	t.printPrompt()

	i := 0
inputLoop:
	for {
		var err error
		line[i], err = t.readByte()
		if err != nil {
			return line[:i], err
		}

		switch line[i] {
		case asciiETX: // Ctrl-C
			i = 0
			t.eraseLine()
			t.printPrompt()
			continue
		case asciiCarriageReturn: // Enter
			t.WriteString(newLine)
			break inputLoop
		case asciiDEL: // Backspace
			i -= 1
			t.eraseLine()
			t.printPrompt()
			t.WriteBytes(line[:i])
			continue
		}

		if t.echo {
			if t.printHex {
				fmt.Fprintf(t.term, "%X ", line[i])
			} else {
				fmt.Fprintf(t.term, "%c", line[i])
			}
		}

		i++
	}

	return line[:i], nil
}

func (t *Terminal) eraseLine() {
	fmt.Fprint(t.term, "\r", vt100EraseToLineEnd)
}

func (t *Terminal) printPrompt() {
	t.WriteBytes(t.prompt)
}

func (t *Terminal) readByte() (byte, error) {
	b := make([]byte, 1)
	_, err := t.term.Read(b)
	return b[0], err
}

func (t *Terminal) WriteBytes(p []byte) (int, error) {
	return t.WriteString(string(p))
}

func (t *Terminal) WriteString(p string) (int, error) {
	return fmt.Fprint(t.term, p)
}

func (t *Terminal) Println(a ...interface{}) {
	fmt.Fprint(t.term, append(a, newLine)...)
}
