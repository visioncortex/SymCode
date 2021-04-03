<div align="center">

  <img src="https://github.com/visioncortex/symcode/raw/master/docs/images/visioncortex-banner.png">
  <h1>SymCode Library</h1>

  <p>
    <strong>The Symbolic Barcode for Humans and Machines</strong>
  </p>

  <h3>
    <a href="https://www.visioncortex.org/symcode-docs">Story</a>
    <span> | </span>
    <a href="https://symcode.visioncortex.org/">Demo</a>
    <span> | </span>
    <a href="https://github.com/visioncortex/acute32">Usage</a>
  </h3>
  <sub>Built with 🦀 by <a href="//www.visioncortex.org/">The Vision Cortex Research Group</a></sub>
</div>

# Synopsis

`symcode` is the programming library providing the infrastructure to design a Symbolic Barcode,
while supporting a demo implementation `Acute32`.

A SymCode is composed of an array of Symbols arranged in a grid structure.

What can be customized:
1. the symbol set
2. shape and locations of finders
3. arrangement (3x3, 4x5 etc) and padding (space between symbols)
4. encoding and error detection scheme (for different payload sizes)

We provide a demo implementation `Acute32` in one configuration:
3x3 -> 5 symbols * 5 bit/sym = 20 bit payload + 5 bit checksum

A second configuration able to encode more bits is also planned:
5x4 -> 16 symbols * 5 bit/sym = 64 bit payload + 16 bit checksum

# Architecture

The `/acute32` modules implements the scanner traits.

The `/interfaces` module defines the abstract concepts of different stages in a scanner and generator:

1. `SymcodeScanner` The scanning pipeline

1. `Finder` To detect finder elements from a color image

1. `Fitter` To find the correct perspective transform from finder candidates

1. `Reader` To scan the image to read out a series of symbols

1. `Decoder` To decode an array of symbols into bit string

1. `Encoder` To encode a bit string into a Symcode Representation

1. `SymcodeGenerator` To generate a Symcode image for a given Symcode representation

1. `Debugger` For use during development to help visualizing the pipeline stages
