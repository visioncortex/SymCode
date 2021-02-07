SymCode is the pure rust crate providing the infrastructure to design a Symbolic Barcode,
while supporting a demo implementation `Acute32`.

A Symcode is composed of an array of Symbols arranged in a grid structure.

What can be customized:
1. the symbol set
2. shape and locations of finders
3. arrangement (3x3, 4x5 etc) and padding (space between symbols)
4. encoding and error detection scheme (due to different payload size)

We provide a demo implementation `Acute32` in two configurations:
1. 3x3 -> 5 symbols * 5 bit/sym = 20 bit payload + 5 bit checksum
2. 5x4 -> 16 symbols * 5 bit/sym = 64 bit payload + 16 bit checksum

```sh
/scanner
	/interface.rs # the SymcodeScanner API
	/... # helper structs
/generator
	/interface.rs # the SymcodeGenerator API
	/... # helper structs
```

The first and foremost structure is the Symcode:
```rust
trait Symbol {
	type Label;

	fn to_label(&self) -> Self::Label; // label is probably a Enum
	fn to_image(&self) -> BinaryImage; // to be used by SymcodeGenerator
}

struct Symcode {
	symbols: Vec<Box<dyn Symbol>>,
}
```

```rust
struct SymcodeScanner {
	config: Rc<SymcodeConfig>,
}

impl SymcodeScanner {
	pub fn scan_and_decode(&self, image: BinaryImage) -> Result<BitVec, Err> {
		decode(scan(BinaryImage)?)
	}
	pub fn scan(&self, image: BinaryImage) -> Result<Symcode, Err>;
	pub fn decode(&self, symcode: Symcode) -> Result<BitVec, Err>;
}
```

```rust
struct SymcodeGenerator {
	config: Rc<SymcodeConfig>, // to be shared with scanner
}

impl SymcodeGenerator {
	pub fn generate(&self, symcode: Symcode) -> BinaryImage;
}
```

```rust
struct SymcodeConfig {
	symbol: Box<dyn Symbol>,
	finder: Box<dyn Finder>,
	encoder: Box<dyn Encoder>,
	// other configurations
}

trait Finder {
	fn to_image(&self) -> BinaryImage; // to be used by SymcodeGenerator
	fn is_finder(&self, shape: Shape) -> bool; // to be used by SymcodeScanner
}

trait Encoder {
	/// this encoder can only encode exactly N bits
	fn length(&self) -> usize;
	/// encode a bit string to a Symcode, panic if input length is not as defined
	fn encode(&self, bits: BitVec) -> Symcode;
}
```