# epochs-cli
Command-line interface to the epochs crate

This takes strings off the command line and tries to interpret them as
dates using the various methods that [the epochs
crate](https://crates.io/crates/epochs) knows.

Install with

```
$ cargo install epochs-cli
```

This installs a binary called just `epochs`.

```
$ epochs --help
epochs-cli 0.2.0
Command line options for epochs

USAGE:
    epochs [FLAGS] [OPTIONS] [candidates]...

FLAGS:
    -d, --debug      Activate debug mode
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose mode (-v, -vv, -vvv, etc.)

OPTIONS:
        --max <max>                        Don't report dates after this [default: 2100-12-31]
        --min <min>                        Don't report dates before this [default: 2000-01-01]
    -o, --output-format <output-format>    Desired format for output [default: text]  [possible values: JSON,
                                           JsonPretty, Text]

ARGS:
    <candidates>...    Strings to test for epochness
```

Give it a number and it tries to interpret it as a date.

```
$ epochs 1234567890

1234567890 Decimal
  cocoa => 2040-02-14T23:31:30
  google calendar => 2007-03-16T23:31:30
  unix => 2009-02-13T23:31:30

```

Use the --max and --min options to change the amount of output.

```
$ epochs 1234567890 --min=1900-01-01 --max=2020-12-31

1234567890 Decimal
  apfs => 1970-01-01T00:00:01.234567890
  google calendar => 2007-03-16T23:31:30
  java => 1970-01-15T06:56:07.890
  mozilla => 1970-01-01T00:20:34.567890
  unix => 2009-02-13T23:31:30

1234567890 Hexadecimal
  apfs => 1970-01-01T00:01:18.187493520
  java => 1972-06-23T22:44:53.520
  mozilla => 1970-01-01T21:43:07.493520
```

You can give it more than one thing to search for at a time.

```
$ epochs 39857.980209 1234567890 33c41a44-6cea-11e7-907b-a6006ad3dba0 

39857.980209 Float
  icq => 2009-02-13T23:31:30.057

1234567890 Decimal
  cocoa => 2040-02-14T23:31:30
  google calendar => 2007-03-16T23:31:30
  unix => 2009-02-13T23:31:30

33c41a44-6cea-11e7-907b-a6006ad3dba0 UUIDv1
  UUIDv1 => 2017-07-20T01:24:40.472634
  windows file => 2035-10-07T01:24:40.472634
```

It uses [serde](https://crates.io/crates/serde) to give the output in
JSON if you prefer.

```
$ epochs 39857.980209 1234567890 33c41a44-6cea-11e7-907b-a6006ad3dba0 --output-format=JsonPretty
[
  {
    "source": "39857.980209",
    "viewed_as": "Float",
    "epochs": {
      "icq": "2009-02-13T23:31:30.057"
    }
  },
  {
    "source": "1234567890",
    "viewed_as": "Decimal",
    "epochs": {
      "cocoa": "2040-02-14T23:31:30",
      "google calendar": "2007-03-16T23:31:30",
      "unix": "2009-02-13T23:31:30"
    }
  },
  {
    "source": "1234567890",
    "viewed_as": "Hexadecimal",
    "epochs": {}
  },
  {
    "source": "33c41a44-6cea-11e7-907b-a6006ad3dba0",
    "viewed_as": "UUIDv1",
    "epochs": {
      "UUIDv1": "2017-07-20T01:24:40.472634"
      "windows file": "2035-10-07T01:24:40.472634"
    }
  }
]
```
