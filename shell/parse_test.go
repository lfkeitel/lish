package shell

import (
	"testing"
)

var parseTests = []struct {
	input    string
	expected []string
}{
	{
		input:    `echo -n "Hello World!"`,
		expected: []string{"echo", "-n", `Hello World!`},
	},
	{
		input:    `/some/bin --output=json`,
		expected: []string{"/some/bin", "--output", "json"},
	},
}

func TestParser(t *testing.T) {
	for i, test := range parseTests {
		out := ParseShellArgs(test.input)
		if !stringSliceEqual(test.expected, out) {
			t.Errorf("Test %d failed. Expected %v, got %v", i, test.expected, out)
		}
	}
}

func stringSliceEqual(a, b []string) bool {
	for i := range a {
		if b[i] != a[i] {
			return false
		}
	}
	return true
}
