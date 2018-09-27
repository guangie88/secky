# secky

[![Build Status](https://travis-ci.org/guangie88/secky.svg?branch=master)](https://travis-ci.org/guangie88/secky)
[![Build status](https://ci.appveyor.com/api/projects/status/m4yhnkqhwco6kexu/branch/master?svg=true)](https://ci.appveyor.com/project/guangie88/secky/branch/master)

**SEC**rets **K**e**Y**ing Rust program to help keying in secrets into file or
piped programs.

The program aims to prevent secrets from being accidentally keyed in shell
directly for shell history, and also prevent mistakes with the whitespaces being
accidentally introduced at the end of the file.

## Notes

The program doesn't allow echoing into TTY for `stdout` unless `-f` is set.

Press CTRL-D to generate EOF char for ending the text input. You will need to
enter it twice if no newline character is entered prior to the EOF char.

Auto-trimming is performed unless trim flags are set.

Run `secky --help` for more program argument details.

## Installation

### Cargo

```bash
cargo install secky
```

### Direct (only for Linux)

```bash
curl -sSf https://raw.githubusercontent.com/guangie88/secky/master/install-linux.sh | sudo sh
```

## Command Examples

### Print to `stdout`

#### `cat` Method

```bash
# Command
secky | cat

# Input
hello(CTRL-D x2)

# `cat` output (no newline at the end)
hello
```

#### Force Stdout flag Method

```bash
# Command
secky -f

# Input
hello(CTRL-D x2)

# Output (no newline at the end)
hello
```

### Print to file

```bash
# Command
secky > secret.txt

# Input
hello(CTRL-D x2)

# Verification command
cat secret.txt

# Output (no newline at the end)
hello
```

### Pipe to other programs

```bash
# Command
secky | md5sum

# Input
hello(CTRL-D x2)

# Output
5d41402abc4b2a76b9719d911017c592  -

# Verification command
echo -n "hello" | md5sum

# Output
5d41402abc4b2a76b9719d911017c592  -
```
