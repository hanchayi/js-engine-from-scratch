use super::value::Value;
use std::collections::HashMap;
pub static PROTOTYPE: &'static str = "prototype";
pub static INSTANCE_PROTOTYPE: &'static str = "__proto__";

pub type ObjectData = HashMap<String, Property>;


/// A Javascript Property AKA The Property Descriptor
/// [[SPEC] - The Property Descriptor Specification Type](https://tc39.github.io/ecma262/#sec-property-descriptor-specification-type)   
/// [[SPEC] - Default Attribute Values](https://tc39.github.io/ecma262/#table-4)
#[derive(Trace, Finalize, Clone, Debug)]
pub struct Property {
    /// If the type of this can be changed and this can be deleted
    pub configurable: bool,
    /// If the property shows up in enumeration of the object
    pub enumerable: bool,
    /// If this property can be changed with an assignment
    pub writable: bool,
    /// The value associated with the property
    pub value: Value,
    pub get: Value,
    pub set: Value,
}

impl Property {
    /// Make a new property with the given value
    pub fn new() -> Property {
        Property {
            configurable: false,
            enumerable: false,
            writable: false,
            value: Value::undefined(),
            get: Value::undefined(),
            set: Value::undefined(),
        }
    }

    pub fn from_value(value: Value) -> Property {
        Property {
            configurable: false,
            enumerable: false,
            writable: false,
            value: value,
            get: Value::undefined(),
            set: Value::undefined(),
        }
    }
}