[![crates.io](https://img.shields.io/crates/v/hcal.svg)](https://crates.io/crates/hcal)

# hcal

A hexadecimal calendar in terminal for programmers

## Installation

`cargo install hcal`

## Usage

```console
> hcal --help
A hexadecimal calendar for terminal

USAGE:
    hcal [OPTIONS] [ARGS]

ARGS:
    <year>     Sets the year
    <month>    Sets the month
    <day>      Sets the day

OPTIONS:
    -3, --unbalanced-ternary    Use ternary representation
    -A, --disbale-all           Disable all font effects
    -b, --balanced-ternary      Use balanced ternary representation
    -d, --disable               Disable day marker
    -e, --easter <year>         Prints the Hex Date of easter.
    -E, --effect                Enable title font effects
    -h, --help                  Print help information
    -T, --transform <date>      Prints the Hex Date of <date>. Needs format of dd-mm-yyyy.
    -V, --version               Print version information
    -W, --no-weekend            Disable weekend marker
    -Y, --disable-year-month    Don't print year and month
```

[![asciicast](https://asciinema.org/a/381715.svg)](https://asciinema.org/a/381715)
