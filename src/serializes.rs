//! Trait for types that can be flattened into query parameters.

use crate::query_value::QueryValue;
use crate::Value;
use std::borrow::Cow;
use std::collections::HashMap;

/// Trait for types that can be flattened into query parameters.
/// Used for ParameterStyle::Flat and ParameterStyle::RepeatList.
///
/// This trait handles the recursive flattening of nested structs and arrays
/// into dot-notation key-value pairs, e.g., `Name.1.Field` for array elements
/// or `Name.Field` for nested objects.
pub trait FlatSerialize {
    /// Append this value's query parameters to the map.
    ///
    /// For scalar types, this inserts a single entry with the given name.
    /// For struct types, this inserts entries for each field with `name.field_name` format.
    /// For array types (with RepeatList style), this inserts entries with `name.{index}` format (1-indexed).
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    );
}

impl FlatSerialize for String {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        params.push((name.to_owned().into(), self.into()));
    }
}

impl FlatSerialize for str {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        params.push((name.to_owned().into(), self.into()));
    }
}

impl FlatSerialize for i32 {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        params.push((name.to_owned().into(), (*self).into()));
    }
}

impl FlatSerialize for i64 {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        params.push((name.to_owned().into(), (*self).into()));
    }
}

impl FlatSerialize for bool {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        params.push((name.to_owned().into(), (*self).into()));
    }
}

impl FlatSerialize for f32 {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        params.push((name.to_owned().into(), self.to_string().into()));
    }
}

impl FlatSerialize for f64 {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        params.push((name.to_owned().into(), self.to_string().into()));
    }
}

impl<T: FlatSerialize> FlatSerialize for Vec<T> {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        for (i, item) in self.iter().enumerate() {
            item.flat_serialize(&format!("{}.{}", name, i + 1), params);
        }
    }
}

impl<T: FlatSerialize> FlatSerialize for Option<T> {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        if let Some(v) = self {
            v.flat_serialize(name, params);
        }
    }
}

/// Trait for types that can be serialized as comma-separated values.
/// Used for ParameterStyle::Simple with array types.
///
/// This trait handles the serialization of arrays into a single comma-separated string,
/// e.g., `["a", "b", "c"]` becomes `"a,b,c"`.
pub trait SimpleSerialize {
    /// Serialize this value as a comma-separated string and insert into the map.
    fn simple_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    );
}

impl SimpleSerialize for Vec<String> {
    fn simple_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        if !self.is_empty() {
            let joined = self.join(",");
            params.push((name.to_owned().into(), joined.into()));
        }
    }
}

impl SimpleSerialize for Vec<i32> {
    fn simple_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        if !self.is_empty() {
            let joined = self
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",");
            params.push((name.to_owned().into(), joined.into()));
        }
    }
}

impl SimpleSerialize for Vec<i64> {
    fn simple_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        if !self.is_empty() {
            let joined = self
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",");
            params.push((name.to_owned().into(), joined.into()));
        }
    }
}

impl SimpleSerialize for Vec<bool> {
    fn simple_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        if !self.is_empty() {
            let joined = self
                .iter()
                .map(|v| if *v { "true" } else { "false" })
                .collect::<Vec<_>>()
                .join(",");
            params.push((name.to_owned().into(), joined.into()));
        }
    }
}

impl SimpleSerialize for Vec<f32> {
    fn simple_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        if !self.is_empty() {
            let joined = self
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",");
            params.push((name.to_owned().into(), joined.into()));
        }
    }
}

impl SimpleSerialize for Vec<f64> {
    fn simple_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        if !self.is_empty() {
            let joined = self
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",");
            params.push((name.to_owned().into(), joined.into()));
        }
    }
}

impl<T: SimpleSerialize> SimpleSerialize for Option<T> {
    fn simple_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        if let Some(v) = self {
            v.simple_serialize(name, params);
        }
    }
}

impl FlatSerialize for Value {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        match self {
            Value::Null => {}
            Value::Bool(v) => {
                params.push((name.to_owned().into(), (*v).into()));
            }
            Value::Integer(v) => {
                params.push((name.to_owned().into(), (*v).into()));
            }
            Value::Float(v) => {
                params.push((name.to_owned().into(), QueryValue::OwnedStr(v.to_string())));
            }
            Value::String(v) => {
                params.push((name.to_owned().into(), v.as_str().into()));
            }
            Value::Array(arr) => {
                for (i, item) in arr.iter().enumerate() {
                    item.flat_serialize(&format!("{}.{}", name, i + 1), params);
                }
            }
            Value::Object(obj) => {
                for (key, value) in obj {
                    value.flat_serialize(&format!("{}.{}", name, key), params);
                }
            }
        }
    }
}

impl<V: FlatSerialize> FlatSerialize for HashMap<String, V> {
    fn flat_serialize<'a>(
        &'a self,
        name: &str,
        params: &mut Vec<(Cow<'static, str>, QueryValue<'a>)>,
    ) {
        for (key, value) in self {
            value.flat_serialize(&format!("{}.{}", name, key), params);
        }
    }
}
