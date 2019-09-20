# Builtin Shell Symbols

## Variables

All environment variables of the shell are exposed as symbols. Changing an
environment variable only changes the variable in the shell, changes are not
reflected to subprocesses or outside processes. To modify an environment
variable for a subprocess, call `(export SYM)`.

- `curr-script-path` - Path of current script file.
- `interactive` - Set True if the shell is ran interactively, False otherwise.
- `last-status` - The exit code of the last command.

## Functions

- `(call)` - Call an external application, default handler for vm symbol not found.
- `(cd NEW-PATH)` - Change current working directory.
- `(exit [CODE])` - Exit shell.
- `(export SYMBOL)` - Export environment variable.
- `(pipe ()[ ()...])` - Connect output and input for a chain of commands.
- `(pwd)` - Return current working directory
- `(unexport SYMBOL)` - Unexport environment variable.
- `(prompt)` - Called on each interactive loop. This function must return a
  string which will be used as the user prompt. Note that multi-line prompts
  have a few bugs.
