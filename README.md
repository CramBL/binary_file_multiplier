# Binary File Multiplier

## Purpose
>Stop me from banging my head against the wall that is my ineffecient shell scripts.

Efficient and easy way of duplicating the contents of a small binary file, and writing that duplicated content to a new file.

## Why?
Good for certain CI pipelines where regression testing for performance is required, but downloading large external files is impractical. 

This is certainly possible in a simple shell script, but making a 240 byte file into a 1 GB file is painfully slow in bash, and I couldn't figure out how to make it faster while keeping the contents intact.

## Quickstart
See help
```shell
$ binmult --help
```

Duplicate a file up to the closest (rounded down) multiple of 100 MiB

```shell
# Any size input duplicated up to 100 MiB (rounded down)
$ binmult input.raw --size 100 -o out.raw
```