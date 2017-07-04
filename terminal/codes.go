package terminal

const (
	asciiETX       = 3 // Ctrl-C
	asciiEOT       = 4 // Ctrl-D
	asciiBackspace = 8
	asciiSUB       = 26 // Ctrl-Z
	asciiESC       = 27
	asciiDEL       = 127

	vt100SeqStart   = '['
	vt100ArrowUp    = 'A'
	vt100ArrowDown  = 'B'
	vt100ArrowRight = 'C'
	vt100ArrowLeft  = 'D'

	asciiCarriageReturn = '\r'
	asciiNewline        = '\n'

	newLine = string(asciiCarriageReturn) + string(asciiNewline)

	vt100EraseToLineEnd = "\033[K"

	vt100MoveArrowRight = "\033[C"
	vt100MoveArrowLeft  = "\033[D"

	vt100ClearScreen = "\033[[2J"
)
