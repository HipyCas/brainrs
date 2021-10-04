# brainrs - :crab: Rust :brain: Brainf\*\*k interpreter

`brainrs` is a simple and blazing fast Brainf\*\*k interpreter written in pure Rust.

**`brainrs` is still WIP, and is by far not complete nor stable, expect errors and bugs**

## Features

- :electron: Core commands ([spec](http://brainfuck.org/brainfuck.html)) + more commands

- :package: Available as library for use inside Rust

- :keyboard: Easy to use CLI

- :zap: Extremely fast with millisecond times

## Usage

`brainrs` reads the code from the standard input, meaning you have to pipe in the file contents. The easiest way is to use the `<` character, like seen below:

```sh
brainrs < file.bf
```

Another option is putting `brainrs` on the recieving side of the pipe, which may allow you to use another command to process the input first, as seen below:

```sh
# Pass one-line script directly in the terminal
echo "+++++>+>+<<[>[>>+<<-]>[>+<<+>-]>[<+>-]<<<-]++++[>>+++++ +++++ ++<<-]>>." | brainrs
# Fetch a file from the internet and execute it
curl https://location.online/of/file.fb | brainrs
```
