use gc::Gc;
use crate::js::function::NativeFunctionData;
use crate::js::object::{Property, PROTOTYPE};
use crate::js::value::{from_value, to_value, ResultValue, Value, ValueData};

/// https://searchfox.org/mozilla-central/source/js/src/vm/StringObject.h#19
pub fn make_string(this: Value, _: Value, args: Vec<Value>) -> ResultValue {
    // If we're constructing a string, we should set the initial length
    // To do this we need to convert the string back to a Rust String, then get the .len()
    // let a: String = from_value(args[0].clone()).unwrap();
    // this.set_field_slice("length", to_value(a.len() as i32));

    this.set_private_field_slice("PrimitiveValue", args[0].clone());
    Ok(this)
}
/// Get a string's length
pub fn get_string_length(this: Value, _: Value, _: Vec<Value>) -> ResultValue {
    let this_str: String =
        from_value(this.get_private_field(String::from("PrimitiveValue"))).unwrap();
    Ok(to_value::<i32>(this_str.len() as i32))
}
/// Get the string value to a primitive string
pub fn to_string(this: Value, _: Value, _: Vec<Value>) -> ResultValue {
    // Get String from String Object and send it back as a new value
    let primitive_val = this.get_private_field(String::from("PrimitiveValue"));
    Ok(to_value(format!("{}", primitive_val).to_string()))
}
pub fn char_at(this: Value, _: Value, args: Vec<Value>) -> ResultValue {
    //          ^^ represents instance       ^^ represents arguments (we only care about the first one in this case)
    // First we get it the actual string a private field stored on the object only the engine has access to.
    // Then we convert it into a Rust String by wrapping it in from_value
    let primitive_val: String =
        from_value(this.get_private_field(String::from("PrimitiveValue"))).unwrap();
    let pos = from_value(args[0].clone()).unwrap();

    // Calling .len() on a string would give the wrong result, as they are bytes not the number of unicode code points
    // Note that this is an O(N) operation (because UTF-8 is complex) while getting the number of bytes is an O(1) operation.
    let length = primitive_val.chars().count();

    // We should return an empty string is pos is out of range
    if pos > length || pos < 0 as usize {
        return Ok(to_value::<String>(String::new()));
    }

    Ok(to_value::<char>(primitive_val.chars().nth(pos).unwrap()))
}
/// Create a new `String` object
pub fn _create(global: Value) -> Value {
    let string = to_value(make_string as NativeFunctionData);
    let proto = ValueData::new_obj(Some(global));
    let prop = Property {
        configurable: false,
        enumerable: false,
        writable: false,
        value: Gc::new(ValueData::Undefined),
        get: to_value(get_string_length as NativeFunctionData),
        set: Gc::new(ValueData::Undefined),
    };
    proto.set_prop_slice("length", prop);
    proto.set_field_slice("toString", to_value(to_string as NativeFunctionData));
    proto.set_field_slice("charAt", to_value(char_at as NativeFunctionData));
    string.set_field_slice(PROTOTYPE, proto);
    string
}
/// Initialise the `String` object on the global object
pub fn init(global: Value) {
    global.set_field_slice("String", _create(global.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_string_constructor_is_function() {
        let global = ValueData::new_obj(None);
        let string_constructor = _create(global);
        assert_eq!(string_constructor.is_function(), true);
    }

}