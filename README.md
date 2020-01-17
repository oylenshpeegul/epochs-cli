# epochs-cli
Command-line interface to the epochs crate

This takes strings off the command line and tries to interpret them as
dates using the various methods that [the epochs
crate](https://crates.io/crates/epochs) knows.

```bash
$ epochs-cli 39857.980209 1234567890 33c41a44-6cea-11e7-907b-a6006ad3dba0 --min=2001-09-11 --max=2020-12-31 

39857.980209 Float
  icq => 2009-02-13T23:31:30.057

1234567890 Decimal
  google calendar => 2007-03-16T23:31:30
  unix => 2009-02-13T23:31:30

33c41a44-6cea-11e7-907b-a6006ad3dba0 UUIDv1
  UUIDv1 => 2017-07-20T01:24:40.472634
```
