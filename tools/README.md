# `flipperzero-tools`

Helper tools for working with the [Flipper Zero](https://flipperzero.one/).

## `serial_cli`

Simple serial client for Flipper Zero.

Will automatically try to find the Flipper Zero port if `--port` is not provided.

```
Usage: serial_cli.exe [OPTIONS]

Options:
  -p, --port <PORT>  Serial port (e.g. `COM3` on Windows or `/dev/ttyUSB0` on Linux)
  -h, --help         Print help information
  -V, --version      Print version information
```

## `storage`

Tool for interacting with storage on the Flipper Zero.

Will automatically try to find the Flipper Zero port if `--port` is not provided.

```
Usage: storage [OPTIONS] [COMMAND]

Commands:
  mkdir    Create directory
  format   Format flash card
  remove   Remove file/directory
  read     Read file
  size     Print size of file (in bytes)
  receive  Receive file
  send     Send file or directory
  list     Recursively list files and dirs
  md5sum   Calculate MD5 hash of remote file
  help     Print this message or the help of the given subcommand(s)

Options:
  -p, --port <PORT>  Serial port (e.g. `COM3` on Windows or `/dev/ttyUSB0` on Linux)
  -h, --help         Print help information
  -V, --version      Print version information
```

This provides the same interface as [`storage.py`](https://github.com/flipperdevices/flipperzero-firmware/blob/dev/scripts/storage.py) in the official firmware.

### Examples

#### Send file to device

```
cargo build --release
target/release/storage send my-app.fap /ext/apps/Examples/my-app.fap
```
