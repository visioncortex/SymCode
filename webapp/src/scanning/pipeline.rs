pub trait ScanningProcessor: {
    /// Type definition of input
    type Input;
    /// Type definition of output
    type Output;
    /// Type definition of parameters
    type Params;

    /// Provide input and params to Processor; returns Err(msg) for invalid input/params
    ///
    /// debug is borrowed because the debug utilities are supposedly shared among processors
    fn process(input: Self::Input, params: &Self::Params) -> Result<Self::Output, &str>;

    /// Validate input and params
    fn valid_input_and_params(input: &Self::Input, params: &Self::Params) -> Result<(), &'static str> {Ok(())}
}