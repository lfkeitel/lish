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

- `(call VAL...)` - Call an external application, default handler for vm symbol
  not found.
- `(capc VAL...)` - Like call but forces a map return with process output and
  status.
- `(cd NEW-PATH)` - Change current working directory.
- `(exit [CODE])` - Exit shell.
- `(export SYMBOL)` - Export environment variable.
- `(unexport SYMBOL)` - Unexport environment variable.
- `(pipe ()[ ()...])` - Connect output and input for a chain of commands.
- `(pwd)` - Return current working directory
- `(prompt)` - Called on each interactive loop. This function must return a
  string which will be used as the user prompt. Note that multi-line prompts
  have a few bugs.
