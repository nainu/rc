# rc

row/column selector for stdin

# How to use

```shell script
$ ps aux | rc -c1 -i1
$ rc -h
rc.rs 0.1

USAGE:
    rc [FLAGS] [OPTIONS]

FLAGS:
    -h, --help        Prints help information
    -j, --join
    -V, --version     Prints version information
    -v, --verbose,    Prints verbosely

OPTIONS:
    -c, --column-index <col_idx>    Index number of columns
    -i <n>                          Number of head lines to ignore
$ ps aux | rc -c0 -i1 | sort | uniq
_analyticsd
_appleevents
_assetcache
_atsserver
_coreaudiod
...
$
```