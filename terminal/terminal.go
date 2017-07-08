package terminal

import (
	"errors"
	"fmt"
	"os"

	term "golang.org/x/crypto/ssh/terminal"
)

type Terminal struct {
	stdin    *os.File
	stdout   *os.File
	echo     bool
	printHex bool

	oldstate *term.State
}

func New() (*Terminal, error) {
	if !term.IsTerminal(int(os.Stdin.Fd())) {
		return nil, errors.New("file descriptor is not a valid terminal")
	}

	return &Terminal{
		stdin:  os.Stdin,
		stdout: os.Stdout,
		echo:   true,
	}, nil
}

func (t *Terminal) SetHexDebug() {
	t.printHex = true
}

func (t *Terminal) EnableRawMode() error {
	var err error
	t.oldstate, err = term.MakeRaw(int(t.stdin.Fd()))
	return err
}

func (t *Terminal) DisableRawMode() error {
	if t.oldstate != nil {
		return term.Restore(int(t.stdin.Fd()), t.oldstate)
	}
	return nil
}

func (t *Terminal) Close() error {
	if t.oldstate != nil {
		return term.Restore(int(t.stdin.Fd()), t.oldstate)
	}
	return nil
}

func (t *Terminal) ReadLine(prompt string) (string, error) {
	return t.read(prompt)
}

func (t *Terminal) ReadPassword(prompt string) (string, error) {
	t.echo = false
	line, err := t.read(prompt)
	t.echo = true
	return line, err
}

func (t *Terminal) read(prompt string) (string, error) {
	line := make([]byte, 1024)
	t.WriteString(prompt)

	i := 0
inputLoop:
	for {
		var err error
		line[i], err = t.readByte()
		if err != nil {
			return string(line[:i]), err
		}

		switch line[i] {
		case asciiETX: // Ctrl-C
			i = 0
			t.eraseLine()
			t.WriteString(prompt)
			continue
		case asciiCarriageReturn: // Enter
			t.WriteString(newLine)
			break inputLoop
		case asciiDEL: // Backspace
			i -= 1
			t.eraseLine()
			t.WriteString(prompt)
			t.WriteBytes(line[:i])
			continue
		}

		if t.echo {
			if t.printHex {
				fmt.Fprintf(t.stdout, "%X ", line[i])
			} else {
				fmt.Fprintf(t.stdout, "%c", line[i])
			}
		}

		i++
	}

	return string(line[:i]), nil
}

func (t *Terminal) eraseLine() {
	fmt.Fprint(t.stdout, "\r", vt100EraseToLineEnd)
}

func (t *Terminal) readByte() (byte, error) {
	b := make([]byte, 1)
	_, err := t.stdin.Read(b)
	return b[0], err
}

func (t *Terminal) WriteBytes(p []byte) (int, error) {
	return t.WriteString(string(p))
}

func (t *Terminal) WriteString(p string) (int, error) {
	return fmt.Fprint(t.stdout, p)
}

func (t *Terminal) Println(a ...interface{}) {
	fmt.Fprint(t.stdout, append(a, newLine)...)
}
