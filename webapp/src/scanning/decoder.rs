pub trait Decoder {
    type EncodedData;

    type DecodedData;
    
    type Err;

    /// Decode the data given how many bits one glyph represents
    fn decode(encoded_data: Self::EncodedData, num_bits: usize) -> Result<Self::DecodedData, Self::Err>;
}