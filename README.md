<h1 align="center">Hark!</h1>
<h4 align="center">Announce when your long running command has finished!</h4>

## Features

<table>
    <tr><td><b>100% Rust</b></td></tr>
    <tr><td><b>Powered by <a href="https://ifttt.com/home">IFTTT</a> for recieving notifications on the go</b></td></tr>
    <tr><td><b>Support for all platforms</b></td></tr>
    <tr><td><b>Automatiaclly times commands</b></td></tr>
    <tr><td><b>Get updated on the ETA, status and more!</b></td></tr>
    <tr><td><b>Miniscule overhead</b></td></tr>
</table>


## Usage

```bash
USAGE:
    hark <KEY> <CMD>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <KEY>       IFTTT API Key
    <CMD>...    Command and arguments to be run
```

## Example

For example, a command like:

```
$ hark a1b2c3apikey rsync -a really/large/dir/ another/dir/
```

Will give you a notification like:

```
"rsync" finished with success after 45.32m (January 8, 20201 at 9:30pm)
```

## Build

First [install](https://www.rust-lang.org/tools/install) Rust (recommended using rustup).

```zsh
$ git clone https://github.com/erismik/hark.git
$ cd hark
$ cargo build --release
```

## Install

Using cargo,

```bash
$ cargo install hark
```

## Contributions

Suggestions, issues, and pull requests are welcome!
