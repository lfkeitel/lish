# LISH

Lish is a unix shell emulator written in Rust. Lish uses a lisp syntax instead
of the traditional POSIX shell.

## Building

Clone the repository then run `cargo build`.

## Using

Run the built binary `./target/debug/lish`. The shell will start with a simple
dollar sign prompt. Type in your command and have fun.

## What can Lish do?

- History (up/down arrow keys)
- Line editing (left/right arrow keys, DEL, Home, End, etc.)
- Ctrl-c to quit current command
- Custom prompt (define a function named `prompt`)
- Startup file (`~/.config/lish/init.lisp`)

## Why Lisp

Lisp is a rather odd choice for a shell language. I'll admit that. I choose it
mainly out of curiosity to see if it could work. For the most part it does.
There are a few times when the syntax gets a little messy, but I tried to design
Lish and the [Lazuli](https://github.com/lfkeitel/lazuli-lisp) language it uses
to be simple for simple commands. Allowing the user to use advanced syntax if
needed but not force it.

For example, when using Lish as a command prompt, the outer most parentheses
aren't necessary. They're added in by the shell. This makes simple commands look
like most other shells. For example the usage of echo doesn't change. `echo Hello` is valid Lish as it is valid POSIX sh. The only difference is internally,
Lish is actually executing `(echo Hello)`.

## Builtins

- `(call VAL...)` - Call an external application, default handler for vm symbol
  not found.
- `(capc VAL...)` - Like call but forces a map return with process output and
  status.
- `(cd NEW-PATH)` - Change current working directory.
- `(exit [CODE])` - Exit shell.
- `(export SYMBOL)` - Export environment variable.
- `(unexport SYMBOL)` - Unexport environment variable.
- `(pipe ()[ ()...])` - Connect output and input for a chain of commands.
- `(| ()[ ()...])` - Alias for `(pipe)`.
- `(pwd)` - Return current working directory
- `(prompt)` - Called on each interactive loop. This function must return a
  string which will be used as the user prompt. Note that multi-line prompts
  have a few bugs.

## TODO

- Tests
- Run commands in the background
- Implement redirection
