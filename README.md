# hexx
Terminal based hex viewer written in Rust

## Example

![Example image](https://github.com/DaBaddest/hexx/blob/main/Images/example1.png)

## Usage
```
A simple hex viewer written in Rust
Usage: hexx [options] filename

Options:
-s, --start:
        Specify the START index, processing of the file will start from here

-e, --end:
        Specify the END index, processing of the file will end here

-l, --lines:
        Specify the number of lines to print

-head, --head:
        Print the first 10 lines of the file

-h, --help:
        Print this help message

Examples:
        hexx -s 100 filename.bin
        hexx --start 10 --end 0x40 filename.bin
```
