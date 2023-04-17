# Nonce Utility
A command-line utility for reading and writing nonces.

## Usage
The utility provides two commands: read and write.

### Read

The read command displays the nonce in the specified or default nonce file.

Usage:

``nonce read [file]``

Arguments:

file - Optional. The file from which to read the nonce. If not provided, the default nonce file will be used.

### Write

Write a new nonce to a file.

##### Arguments

- `<nonce>`: Integer value to write as nonce.

##### Options

- `<file>`: Optional file to write the nonce to.

## Default file location

The default file location is `$HOME/.orga-wallet/nonce`.
