# Emoji Alphabet
## Purpose
The purpose of this project is to allow people to quickly and easily read in a sentence of alphanumeric ascii
characters and output something that can be copied and pasted into slack and produce an arbitrary set of emojis

## Why Rust?
Speed was a concern (see quickly) so I needed something that was web-scale that can also be built
to run in almost any hardware environment (see easily: if I can't run it on a smart-bulb I can't run it easily).
Rust meets those two criteria. It's also a fun learning experience

## Requirements
Rust

## Setup
`git clone`
`cargo install`
`cargo test`

Assuming this all works then you can build the package using `cargo build`

## Example usage
```shell
emojify-sentence -s "foo bar"
:alphabet-yellow-f:alphabet-yellow-o:alphabet-yellow-o        :alphabet-yellow-b:alphabet-yellow-a:alphabet-yellow-r%
```

see `emojify-sentence --help` for more info

## Todo
* Allow args to be passed in specifying output (e.g. `emoji-alphabet --sentence  "foo" --prefix "bar" -modifier "baz" -modifier "quux"` should'
output something like ":bar-baz-f::bar-baz-o::bar-quux-o`)
To that end the following needs to happen:
* Allow "emoji prefix" (`alphabet` in the above example) to be set
* Allow "emoji modifier" (`yellow/white` in the above example) to be set
* Anything else that improves the emojification experience
