# ptimer
A simple command-line timer application.

1. [License](#License)
1. [Dependencies](#Dependencies)
1. [Documentation](#Documentation)
1. [Code Style](#Code-Style)
1. [Building from Source](#Building-from-Source)
1. [Usage](#Usage)

## License

[See the COPYING file](COPYING)

## Dependencies

ptimer has no runtime dependencies, and the only build dependency is Rust.

## Documentation

Documentation for the code can be generated through the `cargo doc --open`
command.

## Code Style

Mostly standard rustfmt style, with the exception of trailing commas.

[See STYLE.md](STYLE.md)

## Building from Source

ptimer is built using Rust, which comes with its own handy cargo command which
are quite useful. If you already have experience with cargo, it will behave
in the expected way.

Building has only been tested on macOS, however it should work on most linux
distributions and probably work on Windows.

ptimer can be built either manually using cargo, or using the `make`
command from the Makefile in the base of the repository.

### Makefile

From the base of the repository just type the command:
```
make ptimer
```

The resulting executable will be in __`./build/ptimer`__. This will be a release
version of the executable and therefore in development it is recommend that
you use `cargo run` to test changes.

There is another make task called `install_bin`, which
copies the resulting binary to `/usr/local/bin/ptimer`. It is not recommended to
run this make task without first knowing that that is exactly where you want it.

### Cargo

To build and run the program, navigate to project root and
run the appropriate cargo command. If you are unfamiliar with cargo, this
is probably:
```
cargo run -- [ put arguments here ]
```

If you want to generate a full executable file you could run:
```
cargo build --release
```

then the resulting executable will be in `[repo_root]/target/release/ptimer`

## Usage

The general structure will look something like this:
```bash
ptimer <time format> <options>
```

Where options and timer format arguments can be intermixed and rearranged

and

Where `<options>` is just a space separated list of standard-format options:
| option | effect |
|--------|--------|
| `--silent` | Will not play a sound after the timer finishes |
| `--exit-on-stop` | The program will exit when timer has stopped instead of showing a time-since-completed timer |

and

Where `<time format>` follows the following format in ebnf:
```ebnf
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

