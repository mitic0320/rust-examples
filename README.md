# rust-examples

Rust examples

## command

Run a command interactively in program. For example, when user run `npm login` command:

1. The shell will first prompt you to input `Username`.
2. After you input your username, then the shell will prompt you to input `Password` and `email`.
3. When you input password and email, then the shell will print successful or errored message.

The `command` example use `thread` to process `stdin/stdout/stderr` concurrently.

## async-command

The async version of interactive command with `async_std`.
