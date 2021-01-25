pub trait Decoder {
    type EncodedData;

    type DecodedData;
    
    type Err;

    fn decode(encoded_data: Self::EncodedData) -> Result<Self::DecodedData, Self::Err>;
}