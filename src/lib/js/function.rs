use super::object::ObjectData;
use super::value::{ResultValue, Value};

pub type FunctionData = fn(Vec<Value>, Value, Value, Value) -> ResultValue;
/// A Javascript function
/// A member of the Object type that may be invoked as a subroutine
/// https://tc39.github.io/ecma262/#sec-terms-and-definitions-function



/// Represents a regular javascript function in memory
/// A member of the Object type that may be invoked as a subroutine
#[derive(Trace, Finalize, Debug)]
pub struct Function {
    /// The fields associated with the function
    pub object: ObjectData,
    pub repr: FunctionData,
    /// The argument names of the function
    pub args: Vec<String>,
}
