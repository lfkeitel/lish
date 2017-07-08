package shell

import (
	"bytes"
	"unicode"
)

func ParseShellArgs(line string) []string {
	if line == "" {
		return []string{}
	}

	runeLine := []rune(line)

	splits := make([]string, 0, 5)

	csplit := make([]rune, 0, 5)
	// Loop over each rune
	for i := 0; i < len(runeLine); i++ {
		crune := runeLine[i]

		if unicode.IsSpace(crune) || crune == '=' {
			if len(csplit) > 0 {
				splits = append(splits, string(csplit))
				csplit = make([]rune, 0, 5)
			}
			continue
		}

		switch crune {
		case '"':
			quotedStr, index := readString(runeLine[i+1:])
			splits = append(splits, quotedStr)
			csplit = make([]rune, 0, 5)
			i += index
		default:
			csplit = append(csplit, crune)
		}
	}

	if len(csplit) > 0 {
		splits = append(splits, string(csplit))
	}

	return splits
}

func readString(line []rune) (string, int) {
	buf := bytes.Buffer{}
	i := 0

	for _, crune := range line {
		i++
		if crune == '"' {
			break
		}
		buf.WriteRune(crune)
	}

	return buf.String(), i
}
