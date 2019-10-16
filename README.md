# idgaf - for when you don't care about the output of a command

`idgaf` is a program that can be used to completely ignore console output
and have it instead redirected into two log files: one for stantdard output and one for standard error.

## Usage

```
idgaf 0.1.0
Mattias Cibien <mattias@mattiascibien.net>

USAGE:
    idgaf.exe [FLAGS] <command>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Shows some debug information

ARGS:
    <command>    The command to execute
```

When invoked it creates two log files in the current directory named <command>.out.log for stdout and <command>.err.log for standard error.
