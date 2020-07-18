# Original implementation

This is the original implementation of a tiny UNIX shell, submitted by Sean Dorward to The International Obfuscated C Code Contest (http://ioccc.org/) in 1990. The submission actually won the "Best Utility" nomation.

If you try to comile it - you are likely to get "void value not ignored as it ought to be" error from the compiler.
To fix it - add a custom `int _exit(int)` function that would do the exit() and return some int value. The original app was written in K&R style, which violates modern standard expectations.

The notes from the author decoded are:

```
This program is a rudimentary shell. It does i/o redirection, pipes
and cd. It flags errors on failed chdir's, open's, creat's
execvp's, fork's and a few syntax errors.

This program is obfuscated in a few notable ways: apart from the
layout (an unformatted (but crunched) version is included for
people who want to put this through cb) it makes clever use of a
write statement, so that the same statement can be used to print
errors and the prompt. By calling the error function with the value
-8, the pointer offset in the expression "?\n$ "-x/4 goes from 0 to
2.  Presto!  A prompt. For errors with numbers smaller than -4
(i.e., UNIX system calls) a question mark is printed.

The error value of chdir is doubled so that we don't exit from the
parent shell on a chdir error (since e() exits on -1 errors only).
All other system call failures exit since they are from subshells.

Recursion is sneakily employed to avoid a second call to fork(),
and the line is parsed in a fairly bizarre fashion:  backwards. The
heart of the program, that is, the part which performs all forks,
execs, opens, etc. is ONE C STATEMENT.

The meta-values array is initialized in a bizarre fashion, and the
subsequent checks for the '<' and '>' are performed in a single
statement using a mask, since you know that '>'&2 is 0, whereas
'<'&2 is 2. Other such micro-obfuscations abound.

Finally, it is notable that the code was hacked for minimality. If
you look at the compressed version, you will be hard-pressed to
eliminate more than a few characters (we can't see how to make it
any smaller!).  550 characters is pretty lean for a shell that does
this much.

BUGS

The syntax of the shell has not been fully explored, but if you try
to redirect in the same direction more than once, only one
redirection is performed. This is a "feature" of the way the line
is parsed; a pointer to the stack of arguments is assigned and an
argument is stolen every time a ">" or "<" is encountered.  The
shell flags an error if no arguments are on the stack. Thus, for
example:
cat > foo > bar
cats to foo, since it was pushed last, but
cat > > foo bar
cats to bar, since bar was pushed under foo. (remember we're
parsing right-left)

Depending on your flavor of UNIX, cd without an argument will
either produce an error or just do nothing.

There is just one error message, the question mark, but hey, that's
all ed does too.
```
