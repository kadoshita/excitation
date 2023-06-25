# excitation
The CLI tool to generate IP addresses for documentation as defined by RFC5737

## Example

```sh
$ ./excitation --help
The CLI tool to generate IP addresses for documentation as defined by RFC5737

Usage: excitation [OPTIONS]

Options:
  -n, --number <NUMBER>  Number of generate addresses [default: 1]
  -b, --block <BLOCK>    Network block [default: 1]
  -h, --help             Print help
  -V, --version          Print version

$ ./excitation -b 3 -n 4
203.0.113.60
203.0.113.15
203.0.113.187
203.0.113.100
```
