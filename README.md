# ptimer
A simple command-line timer application.

## Installation

There are 2 main versions of the program: the C version and the Rust vesrion.
The C version is the original version and is not currently being updated or
supported further. The Rust vesrion is the newest and greatest version of the
program, and is therefore the recommended version.

### Rust Version

The Rust version can be built either manually using cargo, or using the `make`
command from the Makefile in the base of the repository.

#### Makefile

From the base of the repository just type the command:
```
make rust_build
```

The resulting executable will be in __`./build/ptimer`__.

There is another make task called `install_rust_bin`, which simply copies the
resulting binary to `/usr/local/bin/ptimer`. It is not recommended to run this
make task without first knowing that that is exactly where you want it.

#### Cargo

To build and run the program, navigate to `[repo_root]/rust_version/ptimer` and
run the appropriate cargo command. If you are unfamiliar with cargo, this
is probably:
```
cargo run -- [ put arguments here ]
```

### C Version (Deprecated)

To build the C version of the program run the following command:
```
make ptimer
```

and the run use:
```
./build/ptimer
```

There is another make task called `install_bin`, which simple copies the
resulting binary to `/usr/local/bin/ptimer`. It is not recommended to run this
make task without first knowing that that is exactly where you want it.

## Usage

The general structure will look something like this:
```bash
ptimer <time format>
```

Where `<time format>` follows the following format in ebnf:
```text
time_format ::= time_item+
time_item ::= ( number | hour_item | minute_item | second_item )
hour_item ::= "-h " number
minute_item ::= "-m " number
second_item ::= "-s " number
digit ::= (0-9)+
```

### Examples

| command | effect |
|---------|--------|
| `ptimer 34` | Timer for 34 seconds |
| `ptimer 63` | Timer for 1m 3s |
| `ptimer -h 5` | Timer for 5h 0m 0s |
| `ptimer -h 5 -m 3 -s 5` | Timer for 5h 3m 5s |
| `ptimer -h 5 -m 3 -s 62` | Timer for 5h 4m 2s |

