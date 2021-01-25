pub trait Decoder {
    type EncodedData;

    type DecodedData;
    
    type DecodeError;

    fn decode(encoded_data: Self::EncodedData) -> Result<Self::DecodedData, Self::DecodeError>;
}