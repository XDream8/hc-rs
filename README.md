<div align="center">
    <h1>hc-rs -> hosts-creator-rust</h1>
    <p>fetch and merge multiple hosts files</p>
</div>

this is a **WIP**

## whats done

- [x] fetching hosts files
- [x] merging hosts files
- [x] removing duplicate lines
- [x] cli options
- [x] colored output messages
- [ ] configuration file
- [ ] replace with /etc/hosts(i think this should be done manually though)

## building from git source

```
$ git clone https://github.com/XDream8/hc-rs
$ cd hc-rs
$ cargo build --profile optimized
$ ./target/optimized/hc-rs
```

## usage

```sh
$ hc-rs -h
$ hc-rs <urls> <flags>
```

### creating a hosts file

- pass **urls** directly to hc-rs
- you can pass as much urls as you want to

```sh
$ hc-rs https://badmojr.github.io/1Hosts/Pro/hosts.txt https://hosts.oisd.nl
```

### removing duplicate lines

use --rm_duplicate_lines(-rmd) flag to remove duplicate lines from the final file

```sh
$ hc-rs -rmd
```

### setting output filename

use --output(-o) flag to set output filename\
default filename is "hosts"

```sh
$ hc-rs -o new-hosts
```

### aliasing in your shell config

```sh
alias create-hosts='hc-rs -rmd https://badmojr.github.io/1Hosts/Pro/hosts.txt https://hosts.oisd.nl'
```
