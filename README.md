<h1 align="center">Hark!</h1>
<h4 align="center">Announce when your long running command has finished!</h4>

## Features

<table>
    <tr><td><b>100% Rust</b></td></tr>
    <tr><td><b>Powered by <a href="https://ntfy.sh">ntfy.sh</a> for recieving notifications on the go</b></td></tr>
    <tr><td><b>Support for most platforms</b></td></tr>
    <tr><td><b>Automatiaclly times commands</b></td></tr>
    <tr><td><b>Can monitor already started processes</b></td></tr>
    <tr><td><b>Get updated on the ETA, status and more! (coming soon)</b></td></tr>
    <tr><td><b>Miniscule overhead</b></td></tr>
</table>


## Usage

```bash
Usage: hark [OPTIONS] <CMD>...

Arguments:
  <CMD>...  Command and arguments to be run

Options:
  -v, --verbose        Print verbose output
  -t, --topic <TOPIC>  ntfy.sh topic to publish notification to
  -c, --config <FILE>  Sets a custom config file path [default: /etc/hark.toml]
  -s, --save           Save command line settings to config file
  -p, --process <PID>  PID of the process to monitor
  -h, --help           Print help
  -V, --version        Print version
```

## ntfy.sh

Before being able to use `hark`, you'll need to set yourself up with [ntf.sh](ntfy.sh).
Download the app, and then create a topic to use here.
Pass it in using `-t` or [save it to a config file](#config).

## Examples

Use hark to time and launch a command:
```bash
hark rsync -a really/large/dir/ another/dir/
```
Your notification will be something like:
```
"rsync" finished with success after 45.32m (January 8, 20201 at 9:30pm)
```
---
You can use it to attach to an existing process by the processes PID:
```bash
hark -p 2077
```
Your notification will be something like:
```
"rsync" (PID 2077) finished with status "47" after 45.32m (January 8, 20201 at 9:30pm)
```
---
You can use it without any arguments, where `hark` will just send a notification right away:
```bash
dd if=/dev/zero of=/dev/sda1 bs=1M; hark
```
Your notification will be something like:
```
Command finished (January 8, 20201 at 9:30pm)
```


## Config

You can use the configuration file to store your `ntfy.sh topic`, so you don't need to input it everytime you run the command.
Configuration files can be given in any of the following formats: `JSON, YAML, TOML, HJSON`.

An example `.toml` file would look like this:
```toml
topic = "my-topic-here"
```

By default `hark` will look for a configuration file at `/etc/hark.toml`.
This file can be changed via the command line options.
If the specified or default configuration file is not found, `hark` will look for a configuration file in the current directory with the name `hark.*` where `*` is any of the supported configuration types.


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
$ cargo install --path .
```


## Contributions

Suggestions, issues, and pull requests are welcome!
