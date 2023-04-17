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

- `<file>`: Optional file from which to read the nonce.

#### `write`

Write a new nonce to a file.

##### Arguments

- `<nonce>`: Integer value to write as nonce.

##### Options

- `<file>`: Optional file to write the nonce to.

## Default file location

The default file location is `$HOME/.orga-wallet/nonce`.
