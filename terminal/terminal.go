package terminal

import (
	"fmt"
	"os"

	term "golang.org/x/crypto/ssh/terminal"
)

const (
	newLine  = "\r\n"
	asciiEsc = 27
)

type Terminal struct {
	prompt []byte
	term   *os.File
	echo   bool

	state *term.State
}

func NewTerminal(fd uintptr) *Terminal {
	return &Terminal{
		term: os.NewFile(fd, "terminal"),
		echo: true,
	}
}

func (t *Terminal) SetRawMode() error {
	var err error
	t.state, err = term.MakeRaw(int(t.term.Fd()))
	return err
}

func (t *Terminal) Close() error {
	if t.state != nil {
		return term.Restore(int(t.term.Fd()), t.state)
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
	t.Write(t.prompt)

	i := 0
	for {
		var err error
		line[i], err = t.readByte()
		if err != nil {
			return line[:i], err
		}

		if line[i] == 3 {
			i = 0
			fmt.Fprint(t.term, "\r", "\033[K")
			t.Write(t.prompt)
			continue
		}

		if line[i] == 13 {
			fmt.Fprint(t.term, newLine)
			break
		}

		if t.echo {
			fmt.Fprintf(t.term, "%c", line[i])
		}

		i++
	}

	return line[:i], nil
}

func (t *Terminal) readByte() (byte, error) {
	b := make([]byte, 1)
	_, err := t.term.Read(b)
	return b[0], err
}

func (t *Terminal) Write(p []byte) (int, error) {
	return fmt.Fprint(t.term, string(p))
}

func (t *Terminal) Println(a ...interface{}) {
	fmt.Fprint(t.term, append(a, newLine)...)
}
