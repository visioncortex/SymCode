SymCode is the pure rust crate providing the infrastructure to design a Symbolic Barcode,
while supporting a demo implementation `Acute32`.

A Symcode is composed of an array of Symbols arranged in a grid structure.

What can be customized:
1. the symbol set
2. shape and locations of finders
3. arrangement (3x3, 4x5 etc) and padding (space between symbols)
4. encoding and error detection scheme (due to different payload size)

We provide a demo implementation `Acute32` in one configuration:
3x3 -> 5 symbols * 5 bit/sym = 20 bit payload + 5 bit checksum

A second configuration able to encode more bits is also planned:
5x4 -> 16 symbols * 5 bit/sym = 64 bit payload + 16 bit checksum

The `/interfaces` module defines the abstract concepts of different components composing a scanner.

The `/acute32` modules implements this interface. Other implementations can base themselves on it.