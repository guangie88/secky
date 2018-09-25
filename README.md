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

Press CTRL-D to end the text input. Auto-trimming is performed unless trim flags
are set.

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

#### Method 1

Command:

```bash
secky | cat
```

Input entry:

```bash
hello
(CTRL-D)
```

Echo (no newline at the end):

```bash
hello
```

#### Method 2

Command:

```bash
secky -f
```

Input entry:

```bash
hello
(CTRL-D)
```

Echo (no newline at the end):

```bash
hello
```

### Print to file

Command:

```bash
secky > secret.txt
```

Input entry:

```bash
hello
(CTRL-D)
```

Check (no newline at the end):

```bash
cat secret.txt
hello
```

### Pipe to other programs

Command:

```bash
secky | md5sum
```

Input entry:

```bash
hello
(CTRL-D)
```

Echo:

```bash
5d41402abc4b2a76b9719d911017c592  -
```

Check:

```bash
echo -n "hello" | md5sum
5d41402abc4b2a76b9719d911017c592  -
```
