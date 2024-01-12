# rwhich - rust which
## rwhich is a cli utility that finds executable files on the file system. The tool works on all platforms.
### Build
``` cmd
$ cargo build --release
```
### Usage
``` cmd
$ rwhich -h
rwhich returns the pathnames of the executable

Usage: rwhich.exe [OPTIONS] <bin>

Arguments:
  <bin>  The name of binary

Options:
  -a, --all      Print all matching pathnames of binary
  -h, --help     Print help
  -V, --version  Print version
```