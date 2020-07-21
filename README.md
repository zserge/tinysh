# tinysh

> If you hold a UNIX shell to your ear, you can hear the C.

This is a tiny UNIX shell, implemented in Rust and in C.
It is derived from the brilliant IOCCC submission back in 1990 by Sean Dorward. The repository includes the original IOCCC version, as well as the modernized deobfuscated version in C89, and a complete rewrite in Rust.

To run the shell do: `cargo run`. Or, better, `rlwrap cargo run` if you want to have a history of commands.

To run the C version do: `cd c && make && ./tinysh`.

The shell supports:

* Simple commands, i.e. `vim`, `echo hello world` etc.
* Pipelines, i.e. `ls | Cargo | wc -l'.
* File redirection, i.e. `echo hello > x` and `cat < x | grep hello`.

Pretty good for ~100 lines of code in either C or Rust. However, it does not support:

* `>>` append operator.
* `2>` or `2>&1` or anything more complex.
* `&`, although that should be trivial to add.
* Globs, variables, conditionals, loops, functions and it will never be a proper POSIX shell.

Only a toy. Use and explore at your own risk. However, PRs are welcome for bugfixes, or if the additional functionality would not increase the complexity.
