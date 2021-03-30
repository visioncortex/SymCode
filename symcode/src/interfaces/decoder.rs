use bit_vec::BitVec;

type DecodedData = BitVec;

/// To decode an array of symbols into bit string
pub trait Decoder {

    type Symbol;

    type Err;

    /// Attempt to decode the data. If no errors, return a bit string
    fn decode(&self, encoded_data: Vec<Self::Symbol>) -> Result<DecodedData, Self::Err>;

    /// How many bits one symbol encodes
    fn num_bits_per_symbol(&self) -> usize;
}