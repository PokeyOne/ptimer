# ptimer
A simple command-line timer application.

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

