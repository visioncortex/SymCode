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
    fn process(input: Self::Input, params: &Option<Self::Params>) -> Result<Self::Output, &str>;

    /// Validate input
    fn valid_input(input: &Self::Input) -> bool {true}

    /// Validate params
    fn valid_params(params: &Self::Params) -> bool {true}
}