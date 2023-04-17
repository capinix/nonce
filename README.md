# Nonce Utility

A command-line utility for reading and writing nonces.

## Usage

The utility provides two commands: read and write.

### Read
The read command displays the nonce in the specified or default nonce file.

#### Usage:
nonce read [file]



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
