pub trait ScanningProcessor: {
    /// Type definition of input
    type Input;
    /// Type definition of output
    type Output;
    /// Type definition of parameters
    type Params;
    /// Type definition of debug utilities
    type Debug;

    /// Provide input and params to Processor; returns none for invalid input/params
    ///
    /// debug is borrowed because the debug utilities are supposedly shared among processors
    fn process(input: Self::Input, params: Option<Self::Params>, debug: &Option<Self::Debug>) -> Option<Self::Output>;

    /// Validate input
    fn valid_input(input: &Self::Input) -> bool;

    /// Validate params
    fn valid_params(params: &Self::Params) -> bool;
}