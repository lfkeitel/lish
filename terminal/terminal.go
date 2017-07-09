package terminal

import (
	"errors"
	"fmt"
	"io"
	"os"
	"strconv"

	"github.com/lfkeitel/lish/history"

	term "golang.org/x/crypto/ssh/terminal"
)

const (
	lineSize = 1024
)

type FileInput interface {
	Fd() uintptr
	Read(b []byte) (n int, err error)
}

type Terminal struct {
	stdin    FileInput
	stdout   io.Writer
	echo     bool
	printHex bool
	realLine []byte
	//	virtualLine []byte
	prompt    string
	cursorPos int

	oldstate *term.State

	hist         *history.History
	currHistItem int
}

func New() (*Terminal, error) {
	if !term.IsTerminal(int(os.Stdin.Fd())) {
		return nil, errors.New("file descriptor is not a valid terminal")
	}

	return &Terminal{
		stdin:  os.Stdin,
		stdout: os.Stdout,
		echo:   true,
		hist:   history.New(),
	}, nil
}

func (t *Terminal) WithHistory(h *history.History) {
	t.hist = h
	t.currHistItem = h.Len() - 1
}

func (t *Terminal) AddHistory(item string) {
	t.hist.Add(item)
	t.currHistItem++
}

func (t *Terminal) SetHexDebug() {
	t.printHex = true
}

func (t *Terminal) EnableRawMode() error {
	var err error
	if t.oldstate == nil {
		t.oldstate, err = term.MakeRaw(int(t.stdin.Fd()))
	}
	return err
}

func (t *Terminal) DisableRawMode() error {
	var err error
	if t.oldstate != nil {
		err = term.Restore(int(t.stdin.Fd()), t.oldstate)
		t.oldstate = nil
	}
	return err
}

func (t *Terminal) Close() error {
	var err error
	if t.oldstate != nil {
		err = term.Restore(int(t.stdin.Fd()), t.oldstate)
		t.oldstate = nil
	}
	return err
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
	t.prompt = prompt
	t.eraseLine()
	t.printLine()

inputLoop:
	for {
		nextByte, err := t.readByte()
		if err != nil {
			return string(t.realLine), err
		}

		if t.printHex {
			fmt.Fprintf(t.stdout, "%X ", nextByte)
			t.realLine = append(t.realLine, nextByte)
			if nextByte == asciiCarriageReturn {
				t.WriteString(newLine)
				break inputLoop
			}
			continue
		}

		switch nextByte {
		case asciiETX: // Ctrl-C
			t.eraseLine()
			t.WriteString(newLine)
			t.printLine()
			continue
		case asciiCarriageReturn: // Enter
			t.WriteString(newLine)
			break inputLoop
		case asciiDEL: // Backspace
			t.backspaceChar()
			continue
		case asciiESC: // Escape sequence
			t.handleEscape()
			continue
		}

		if t.cursorPos == len(t.realLine) {
			t.realLine = append(t.realLine, nextByte)
		} else {
			// Avoid a second allocation by using copy instead of two appends
			// https://github.com/golang/go/wiki/SliceTricks#insert
			t.realLine = append(t.realLine, 0)
			copy(t.realLine[t.cursorPos+1:], t.realLine[t.cursorPos:])
			t.realLine[t.cursorPos] = nextByte
		}

		t.cursorPos++

		if t.echo {
			t.printLine()
		}
	}

	return string(t.realLine), nil
}

func (t *Terminal) handleEscape() {
	bracket, err := t.readByte()
	if err != nil || bracket != vt100SeqStart {
		return
	}

	nextByte, err := t.readByte()
	if err != nil {
		return
	}

	switch nextByte {
	case vt100ArrowUp:
		t.lastHistory()
	case vt100ArrowDown:
		t.nextHistory()
	case vt100ArrowRight:
		t.moveRight()
	case vt100ArrowLeft:
		t.moveLeft()
	case '3':
		nextByte, _ = t.readByte()
		if nextByte == '~' {
			t.deleteCursorChar()
		}
	case 'F':
		t.moveToEnd()
	case 'H':
		t.moveToHome()
	}
}

func (t *Terminal) lastHistory() {
	if t.currHistItem > 0 {
		t.currHistItem--
	}
	t.realLine = []byte(t.hist.Get(t.currHistItem))
	t.cursorPos = len(t.realLine)
	t.printLine()
}

func (t *Terminal) nextHistory() {
	if t.currHistItem < t.hist.Len() {
		t.currHistItem++
	}

	if t.currHistItem == t.hist.Len() {
		t.eraseLine()
		t.printLine()
		return
	}

	t.realLine = []byte(t.hist.Get(t.currHistItem))
	t.cursorPos = len(t.realLine)
	t.printLine()
}

func (t *Terminal) moveToEnd() {
	t.cursorPos = len(t.realLine)
	t.printLine()
}

func (t *Terminal) moveToHome() {
	t.cursorPos = 0
	t.WriteString("\r")
	t.moveCursorRight(len(t.prompt))
}

func (t *Terminal) moveRight() {
	if t.cursorPos == len(t.realLine) {
		return
	}
	t.cursorPos++
	t.moveCursorRight(1)
}

func (t *Terminal) moveLeft() {
	if t.cursorPos == 0 {
		return
	}
	t.cursorPos--
	t.moveCursorLeft(1)
}

func (t *Terminal) backspaceChar() {
	if t.cursorPos == 0 {
		return
	}

	t.realLine = append(t.realLine[:t.cursorPos-1], t.realLine[t.cursorPos:]...)
	t.cursorPos--
	t.printLine()
}

func (t *Terminal) deleteCursorChar() {
	if t.cursorPos == len(t.realLine) {
		return
	}

	t.realLine = append(t.realLine[:t.cursorPos], t.realLine[t.cursorPos+1:]...)
	t.printLine()
}

func (t *Terminal) eraseLine() {
	t.realLine = make([]byte, 0, lineSize)
	t.cursorPos = 0
}

func (t *Terminal) readByte() (byte, error) {
	b := make([]byte, 1)
	_, err := t.stdin.Read(b)
	return b[0], err
}

func (t *Terminal) printLine() {
	t.WriteString("\r")
	t.WriteString(vt100EraseToLineEnd)
	t.WriteString(t.prompt)
	t.WriteBytes(t.realLine)
	if t.cursorPos != len(t.realLine) {
		t.moveCursorLeft(len(t.realLine) - t.cursorPos)
	}
}

func (t *Terminal) moveCursorLeft(spaces int) {
	t.WriteString("\033[" + strconv.Itoa(spaces) + "D")
}

func (t *Terminal) moveCursorRight(spaces int) {
	t.WriteString("\033[" + strconv.Itoa(spaces) + "C")
}

func (t *Terminal) WriteBytes(p []byte) (int, error) {
	return t.WriteString(string(p))
}

func (t *Terminal) WriteString(p string) (int, error) {
	return fmt.Fprint(t.stdout, p)
}

func (t *Terminal) Println(a ...interface{}) (int, error) {
	return fmt.Fprint(t.stdout, append(a, newLine)...)
}
