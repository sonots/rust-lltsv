# lltsv

A CLI tool to list specified keys of LTSV (Labeled Tab Separated Values)

# Description

`lltsv` is a command line tool written in golang to list specified keys of LTSV (Labeled Tab Separated Values) text.

Example 1:

```bash
$ echo "foo:aaa\tbar:bbb\tbaz:ccc" | lltsv -k foo,bar
foo:aaa   bar:bbb
```

The output is colorized as default when you outputs to a terminal.
The coloring is disabled if you pipe or redirect outputs.

Example 2:

```bash
$ echo "foo:aaa\tbar:bbb\tbaz:ccc" | lltsv -k foo,bar -K
aaa       bbb
```

Eliminate labels with `-K` option.

Example 3:

```bash
$ lltsv -k foo,bar -K file*.log
```

Specify input files as arguments.

**How Useful?**

LTSV format is not `awk` friendly (I think), but `lltsv` can help it:

```bash
$ echo -e "time:2014-08-13T14:10:10Z\tstatus:200\ntime:2014-08-13T14:10:12Z\tstatus:500" \
  | lltsv -k time,status -K | awk '$2 == 500'
2014-08-13T14:10:12Z    500
```

Useful!

## Installation

Executable binaries are available at [releases](https://github.com/sonots/rust-lltsv/releases).

For example, for linux x86_64,

```bash
$ wget https://github.com/sonots/rust-lltsv/releases/download/v0.3.0/lltsv_linux_amd64 -O lltsv
$ chmod a+x lltsv
```

## Usage

```
$ lltsv -h
NAME:
   lltsv - List specified keys of LTSV (Labeled Tab Separated Values)

USAGE:
   lltsv [global options] command [command options] [arguments...]

VERSION:
   0.0.1

AUTHOR(S):
   sonots <sonots@gmail.com>

COMMANDS:
   help, h      Shows a list of commands or help for one command

GLOBAL OPTIONS:
   --key, -k                                            keys to output (multiple keys separated by ,)
   --no-key, -K                                         output without keys (and without color)
   --ignore-key value, -i value                         ignored keys to output (multiple keys separated by ,)
   --help, -h                                           show help
   --version, -v                                        print the version
```

## ToDo

1. write tests

## Build

To build, use `cargo build`

```
$ git clone git@github.com:sonots/rust-lltsv
$ cd rust-lltsv
$ cargo build
```

## Contribution

1. Fork (https://github.com/sonots/rust-lltsv/fork)
2. Create a feature branch
3. Commit your changes
4. Rebase your local changes against the master branch
7. Create new Pull Request

## Copyright

See [LICENSE](./LICENSE)

