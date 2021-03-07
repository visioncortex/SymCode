use bit_vec::BitVec;

type DecodedData = BitVec;

pub trait Decoder {

    type Symbol;

    type Err;

    /// Decode the data
    fn decode(&self, encoded_data: Vec<Self::Symbol>) -> Result<DecodedData, Self::Err>;

    /// How many bits one symbol encodes
    fn num_bits_per_symbol(&self) -> usize;
}