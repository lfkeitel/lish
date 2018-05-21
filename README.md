# LISH

Lish is a unix shell emulator written in Go. I made it to learn about terminal manipulation and processing.

## Building

Clone the repository then run `make`.

## Using

Run the built binary `./lish`. The shell will start with a simple dollar sign prompt. Type in your command and have fun.

## What can Lish do?

- History (up/down arrow keys)
- Line editing (left/right arrow keys, DEL, Home, End, etc.)
- Ctrl-c to quit current command
- Custom prompt (set the variable `LISH_PROMPT`)

## Builtins

- `pwd` - Print current working directory
- `cd` - Change directories
- `def` - Set environment variables: `def name "value"`
- `exit` - Exit Lish

## TODO

- Tests
- Run commands in the background
- Implement pipes and redirections
- Implement a scripting language (based on Nitrogen)
  - If statements
  - Loops
  - Functions
  - Arrays
  - Hashmaps
  - Standard library
    - Rather small, just a few necessities I find missing in Bash/ZSH
- Implement compiled versions of scripts
