use gc::Gc;

use super::value::{Value, ValueData, ResultValue, to_value};


/// 创建一个boolean
pub fn make_boolean(_: Vec<Value>, _: Value, _: Value, this: Value) -> ResultValue {
    Ok(Gc::new(ValueData::Undefined))
}

/// 创建一个Boolean
pub fn _create(global: Value) -> Value {
    let boolean = to_value(make_boolean);
    boolean
}

pub fn init(global: Value) {
    let global_ptr = global.borrow();
    global_ptr.set_field_slice("Boolean", _create(global));
}