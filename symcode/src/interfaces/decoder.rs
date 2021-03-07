use bit_vec::BitVec;

type DecodedData = BitVec;

pub trait Decoder {

    type Symbol;

    type Err;

    /// Decode the data given how many bits one glyph represents
    fn decode(encoded_data: Vec<Self::Symbol>, num_bits: usize) -> Result<DecodedData, Self::Err>;
}