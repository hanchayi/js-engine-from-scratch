use gc::{Gc, GcCell};
use super::function::Function;
use super::object::{ObjectData, Property, INSTANCE_PROTOTYPE, PROTOTYPE};
use std::collections::HashMap;
use std::str::FromStr;
use serde_json::map::Map;
use serde_json::Number as JSONNumber;
use serde_json::Value as JSONValue;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::iter::FromIterator;
use std::ops::Deref;
use std::f64::NAN;

/// The result of a Javascript expression is represented like this so it can succeed (`Ok`) or fail (`Err`)
pub type ResultValue = Result<Value, Value>;
/// A Garbage-collected Javascript value as represented in the interpreter
#[derive(Trace, Finalize, Clone, Debug)]
pub struct Value {
    /// The garbage-collected pointer
    pub ptr: Gc<ValueData>,
}

/// A Javascript value
#[derive(Trace, Finalize, Debug)]
pub enum ValueData {
    /// `null` - A null value, for when a value doesn't exist
    Null,
    /// `undefined` - An undefined value, for when a field or index doesn't exist
    Undefined,
    /// `boolean` - A `true` / `false` value, for if a certain criteria is met
    Boolean(bool),
    /// `String` - A UTF-8 string, such as `"Hello, world"`
    String(String),
    /// `Number` - A 64-bit floating point number, such as `3.1415`
    Number(f64),
    /// `Number` - A 32-bit integer, such as `42`
    Integer(i32),
    /// `Object` - An object, such as `Math`, represented by a binary tree of string keys to Javascript values
    Object(GcCell<ObjectData>),
    /// `Function` - A runnable block of code, such as `Math.sqrt`, which can take some variables and return a useful value or act upon an object
    Function(GcCell<Function>),
}

impl Value {
    /// Returns a new empty object
    pub fn new_obj(global: Option<Value>) -> Value {
        let mut obj: ObjectData = HashMap::new();
        if global.is_some() {
            let obj_proto = global
                .unwrap()
                .get_field_slice("Object")
                .get_field_slice(PROTOTYPE);
            obj.insert(
                INSTANCE_PROTOTYPE.to_string(),
                Property::from_value(obj_proto),
            );
        }
        Value {
            ptr: Gc::new(ValueData::Object(GcCell::new(obj))),
        }
    }

    /// Returns true if the value is an object
    pub fn is_object(&self) -> bool {
        match *self.ptr {
            ValueData::Object(_) => true,
            _ => false,
        }
    }

    /// Returns true if the value is undefined
    pub fn is_undefined(&self) -> bool {
        match *self.ptr {
            ValueData::Undefined => true,
            _ => false,
        }
    }

    /// Returns true if the value is null
    pub fn is_null(&self) -> bool {
        match *self.ptr {
            ValueData::Null => true,
            _ => false,
        }
    }

    /// Returns true if the value is null or undefined
    pub fn is_null_or_undefined(&self) -> bool {
        match *self.ptr {
            ValueData::Null | ValueData::Undefined => true,
            _ => false,
        }
    }

    /// Returns true if the value is a 64-bit floating-point number
    pub fn is_double(&self) -> bool {
        match *self.ptr {
            ValueData::Number(_) => true,
            _ => false,
        }
    }

    /// Returns true if the value is a string
    pub fn is_string(&self) -> bool {
        match *self.ptr {
            ValueData::String(_) => true,
            _ => false,
        }
    }

    /// Returns true if the value is true
    /// [toBoolean](https://tc39.github.io/ecma262/#sec-toboolean)
    pub fn is_true(&self) -> bool {
        match *self.ptr {
            ValueData::Object(_) => true,
            ValueData::String(ref s) if !s.is_empty() => true,
            ValueData::Number(n) if n >= 1.0 && n % 1.0 == 0.0 => true,
            ValueData::Integer(n) if n > 1 => true,
            ValueData::Boolean(v) => v,
            _ => false,
        }
    }

    /// Converts the value into a 64-bit floating point number
    pub fn to_num(&self) -> f64 {
        match *self.ptr {
            ValueData::Object(_) | ValueData::Undefined | ValueData::Function(_) => NAN,
            ValueData::String(ref str) => match FromStr::from_str(str) {
                Ok(num) => num,
                Err(_) => NAN,
            },
            ValueData::Number(num) => num,
            ValueData::Boolean(true) => 1.0,
            ValueData::Boolean(false) | ValueData::Null => 0.0,
            ValueData::Integer(num) => num as f64,
        }
    }

    /// Converts the value into a 32-bit integer
    pub fn to_int(&self) -> i32 {
        match *self.ptr {
            ValueData::Object(_)
            | ValueData::Undefined
            | ValueData::Null
            | ValueData::Boolean(false)
            | ValueData::Function(_) => 0,
            ValueData::String(ref str) => match FromStr::from_str(str) {
                Ok(num) => num,
                Err(_) => 0,
            },
            ValueData::Number(num) => num as i32,
            ValueData::Boolean(true) => 1,
            ValueData::Integer(num) => num,
        }
    }

    /// Resolve the property in the object
    /// Returns a copy of the Property
    pub fn get_prop(&self, field: String) -> Option<Property> {
        let obj: ObjectData = match *self.ptr {
            ValueData::Object(ref obj) => {
                let hash = obj.clone();
                hash.into_inner()
            }
            // Accesing .object on borrow() seems to automatically dereference it, so we don't need the *
            // ValueData::Function(ref func) => func.clone().object,
            _ => return None,
        };
        match obj.get(&field) {
            Some(val) => Some(val.clone()),
            None => match obj.get(&PROTOTYPE.to_string()) {
                Some(prop) => prop.value.get_prop(field),
                None => None,
            },
        }
    }

    /// Resolve the property in the object and get its value, or undefined if this is not an object or the field doesn't exist
    pub fn get_field(&self, field: String) -> Value {
        match self.get_prop(field) {
            Some(prop) => prop.value.clone(),
            None => Value {
                ptr: Gc::new(ValueData::Undefined),
            },
        }
    }

    /// Resolve the property in the object and get its value, or undefined if this is not an object or the field doesn't exist
    pub fn get_field_slice<'a>(&self, field: &'a str) -> Value {
        self.get_field(field.to_string())
    }

    /// Set the field in the value
    pub fn set_field(&self, field: String, val: Value) -> Value {
        match *self.ptr {
            ValueData::Object(ref obj) => {
                obj.borrow_mut()
                    .insert(field.clone(), Property::from_value(val.clone()));
            }
            ValueData::Function(ref func) => {
                func.borrow_mut()
                    .object
                    .insert(field.clone(), Property::from_value(val.clone()));
            }
            _ => (),
        }
        val
    }

    /// Set the field in the value
    pub fn set_field_slice<'a>(&self, field: &'a str, val: Value) -> Value {
        self.set_field(field.to_string(), val)
    }

    /// Set the property in the value
    pub fn set_prop(&self, field: String, prop: Property) -> Property {
        match *self.ptr {
            ValueData::Object(ref obj) => {
                obj.borrow_mut().insert(field.clone(), prop.clone());
            }
            ValueData::Function(ref func) => {
                func.borrow_mut().object.insert(field.clone(), prop.clone());
            }
            _ => (),
        }
        prop
    }

    /// Convert from a JSON value to a JS value
    pub fn from_json(json: JSONValue) -> ValueData {
        match json {
            JSONValue::Number(v) => ValueData::Number(v.as_f64().unwrap()),
            JSONValue::String(v) => ValueData::String(v),
            JSONValue::Bool(v) => ValueData::Boolean(v),
            JSONValue::Array(vs) => {
                let mut i = 0;
                let mut data: ObjectData = FromIterator::from_iter(vs.iter().map(|json| {
                    i += 1;
                    (
                        (i - 1).to_string().to_string(),
                        Property::from_value(to_value(json.clone())),
                    )
                }));
                data.insert(
                    "length".to_string(),
                    Property::from_value(to_value(vs.len() as i32)),
                );
                ValueData::Object(GcCell::new(data))
            }
            JSONValue::Object(obj) => {
                let data: ObjectData = FromIterator::from_iter(obj.iter().map(|(key, json)| {
                    (key.clone(), Property::from_value(to_value(json.clone())))
                }));
                ValueData::Object(GcCell::new(data))
            }
            JSONValue::Null => ValueData::Null,
        }
    }

    fn to_json(&self) -> JSONValue {
        match *self.ptr {
            ValueData::Null | ValueData::Undefined => JSONValue::Null,
            ValueData::Boolean(b) => JSONValue::Bool(b),
            ValueData::Object(ref obj) => {
                let mut nobj = Map::new();
                for (k, v) in obj.borrow().iter() {
                    if k != INSTANCE_PROTOTYPE {
                        nobj.insert(k.clone(), v.value.to_json());
                    }
                }
                JSONValue::Object(nobj)
            }
            ValueData::String(ref str) => JSONValue::String(str.clone()),
            ValueData::Number(num) => JSONValue::Number(JSONNumber::from_f64(num).unwrap()),
            ValueData::Integer(val) => JSONValue::Number(JSONNumber::from(val)),
            ValueData::Function(_) => JSONValue::Null,
        }
    }

    /// Get the type of the value
    pub fn get_type(&self) -> &'static str {
        match *self.ptr {
            ValueData::Number(_) | ValueData::Integer(_) => "number",
            ValueData::String(_) => "string",
            ValueData::Boolean(_) => "boolean",
            ValueData::Null => "null",
            ValueData::Undefined => "undefined",
            _ => "object",
        }
    }

    /// Get the value for undefined
    pub fn undefined() -> Value {
        Value {
            ptr: Gc::new(ValueData::Undefined),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self.ptr {
            ValueData::Null => write!(f, "null"),
            ValueData::Undefined => write!(f, "undefined"),
            ValueData::Boolean(v) => write!(f, "{}", v),
            ValueData::String(ref v) => write!(f, "{}", v),
            ValueData::Number(v) => write!(
                f,
                "{}",
                match v {
                    // https://tc39.github.io/ecma262/#sec-tostring-applied-to-the-number-type
                    _ if v.is_nan() => "NaN".to_string(),
                    _ if v.is_infinite() && v.is_sign_positive() => "Infinity".to_string(),
                    _ if v.is_infinite() && v.is_sign_negative() => "-Infinity".to_string(),
                    _ => v.to_string(),
                }
            ),
            ValueData::Object(ref v) => {
                write!(f, "{}", "{")?;
                match v.borrow().iter().last() {
                    Some((last_key, _)) => {
                        for (key, val) in v.borrow().iter() {
                            write!(f, "{}: {}", key, val.value)?;
                            if key != last_key {
                                write!(f, "{}", ", ")?;
                            }
                        }
                    }
                    None => (),
                }
                write!(f, "{}", "}")
            }
            ValueData::Integer(v) => write!(f, "{}", v),
            ValueData::Function(ref v) => write!(f, "function({})", v.borrow().args.join(", ")),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self.ptr.clone().deref(), other.ptr.clone().deref()) {
            // TODO: fix this
            // _ if self.ptr.to_inner() == &other.ptr.to_inner() => true,
            _ if self.is_null_or_undefined() && other.is_null_or_undefined() => true,
            (ValueData::String(_), _) | (_, ValueData::String(_)) => {
                self.to_string() == other.to_string()
            }
            (ValueData::Boolean(a), ValueData::Boolean(b)) if a == b => true,
            (ValueData::Number(a), ValueData::Number(b))
                if a == b && !a.is_nan() && !b.is_nan() =>
            {
                true
            }
            (ValueData::Number(a), _) if *a == other.to_num() => true,
            (_, ValueData::Number(a)) if *a == self.to_num() => true,
            (ValueData::Integer(a), ValueData::Integer(b)) if a == b => true,
            _ => false,
        }
    }
}

/// Conversion to Javascript values from Rust values
pub trait ToValue {
    /// Convert this value to a Rust value
    fn to_value(&self) -> Value;
}
/// Conversion to Rust values from Javascript values
pub trait FromValue {
    /// Convert this value to a Javascript value
    fn from_value(value: Value) -> Result<Self, &'static str>
    where
        Self: Sized;
}

impl ToValue for String {
    fn to_value(&self) -> Value {
        Value {
            ptr: Gc::new(ValueData::String(self.clone())),
        }
    }
}

impl FromValue for String {
    fn from_value(v: Value) -> Result<String, &'static str> {
        Ok(v.to_string())
    }
}

impl<'s> ToValue for &'s str {
    fn to_value(&self) -> Value {
        Value {
            ptr: Gc::new(ValueData::String(String::from_str(*self).unwrap())),
        }
    }
}

impl ToValue for char {
    fn to_value(&self) -> Value {
        Value {
            ptr: Gc::new(ValueData::String(self.to_string())),
        }
    }
}
impl FromValue for char {
    fn from_value(v: Value) -> Result<char, &'static str> {
        Ok(v.to_string().chars().next().unwrap())
    }
}

impl ToValue for f64 {
    fn to_value(&self) -> Value {
        Value {
            ptr: Gc::new(ValueData::Number(*self)),
        }
    }
}
impl FromValue for f64 {
    fn from_value(v: Value) -> Result<f64, &'static str> {
        Ok(v.to_num())
    }
}

impl ToValue for i32 {
    fn to_value(&self) -> Value {
        Value {
            ptr: Gc::new(ValueData::Integer(*self)),
        }
    }
}
impl FromValue for i32 {
    fn from_value(v: Value) -> Result<i32, &'static str> {
        Ok(v.to_int())
    }
}

impl ToValue for bool {
    fn to_value(&self) -> Value {
        Value {
            ptr: Gc::new(ValueData::Boolean(*self)),
        }
    }
}
impl FromValue for bool {
    fn from_value(v: Value) -> Result<bool, &'static str> {
        Ok(v.is_true())
    }
}

impl<'s, T: ToValue> ToValue for &'s [T] {
    fn to_value(&self) -> Value {
        let mut arr = HashMap::new();
        let mut i = 0;
        for item in self.iter() {
            arr.insert(i.to_string(), Property::from_value(item.to_value()));
            i += 1;
        }
        to_value(arr)
    }
}
impl<T: ToValue> ToValue for Vec<T> {
    fn to_value(&self) -> Value {
        let mut arr = HashMap::new();
        let mut i = 0;
        for item in self.iter() {
            arr.insert(i.to_string(), Property::from_value(item.to_value()));
            i += 1;
        }
        to_value(arr)
    }
}

impl<T: FromValue> FromValue for Vec<T> {
    fn from_value(v: Value) -> Result<Vec<T>, &'static str> {
        let len = v.get_field_slice("length").to_int();
        let mut vec = Vec::with_capacity(len as usize);
        for i in 0..len {
            vec.push(from_value(v.get_field(i.to_string()))?)
        }
        Ok(vec)
    }
}

impl ToValue for ObjectData {
    fn to_value(&self) -> Value {
        Value {
            ptr: Gc::new(ValueData::Object(GcCell::new(self.clone()))),
        }
    }
}
impl FromValue for ObjectData {
    fn from_value(v: Value) -> Result<ObjectData, &'static str> {
        match *v.ptr {
            ValueData::Object(ref obj) => {
                let obj_data = obj.clone().into_inner();
                Ok(obj_data)
            }
            ValueData::Function(ref func) => Ok(func.borrow().object.clone()),
            _ => Err("Value is not a valid object"),
        }
    }
}

impl ToValue for JSONValue {
    fn to_value(&self) -> Value {
        Value {
            ptr: Gc::new(Value::from_json(self.clone())),
        }
    }
}
impl FromValue for JSONValue {
    fn from_value(v: Value) -> Result<JSONValue, &'static str> {
        Ok(v.to_json())
    }
}

impl ToValue for () {
    fn to_value(&self) -> Value {
        Value {
            ptr: Gc::new(ValueData::Null),
        }
    }
}
impl FromValue for () {
    fn from_value(_: Value) -> Result<(), &'static str> {
        Ok(())
    }
}

impl<T: ToValue> ToValue for Option<T> {
    fn to_value(&self) -> Value {
        match *self {
            Some(ref v) => v.to_value(),
            None => Value {
                ptr: Gc::new(ValueData::Null),
            },
        }
    }
}
impl<T: FromValue> FromValue for Option<T> {
    fn from_value(value: Value) -> Result<Option<T>, &'static str> {
        Ok(if value.is_null_or_undefined() {
            None
        } else {
            Some(FromValue::from_value(value)?)
        })
    }
}

/// A utility function that just calls FromValue::from_value
pub fn from_value<A: FromValue>(v: Value) -> Result<A, &'static str> {
    FromValue::from_value(v)
}

/// A utility function that just calls ToValue::to_value
pub fn to_value<A: ToValue>(v: A) -> Value {
    v.to_value()
}
