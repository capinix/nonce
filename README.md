# Nonce Utility

A command-line tool for managing a nonce file.

## Features

- Read the current nonce from a file
- Write a new nonce to a file

## Usage

nonce read [OPTIONS]

nonce write <nonce> [OPTIONS]

### Subcommands

#### `read`

Read the current nonce from a file.

##### Options

- `--file <file>`: Optional file from which to read the nonce. If not provided, the tool will read from the default file location.

#### `write`

Write a new nonce to a file.

##### Arguments

- `<nonce>`: Integer value to write as nonce.

##### Options

- `--file <file>`: Optional file to write the nonce to. If not provided, the tool will write to the default file location.

## Default file location

The default file location is `$HOME/.orga-wallet/nonce`.
