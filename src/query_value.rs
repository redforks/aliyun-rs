use std::borrow::Cow;

pub enum QueryValue<'a> {
    Str(&'a str),
    OwnedStr(String),
    I64(i64),
    I32(i32),
    Bool(bool),
    Json(serde_json::Value),
}

impl From<String> for QueryValue<'_> {
    fn from(value: String) -> Self {
        Self::OwnedStr(value)
    }
}

impl<'a> From<&'a String> for QueryValue<'a> {
    fn from(value: &'a String) -> Self {
        Self::Str(value)
    }
}

impl<'a> From<&'a str> for QueryValue<'a> {
    fn from(value: &'a str) -> Self {
        Self::Str(value)
    }
}

impl From<i64> for QueryValue<'_> {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl From<&i64> for QueryValue<'_> {
    fn from(value: &i64) -> Self {
        Self::I64(*value)
    }
}

impl From<i32> for QueryValue<'_> {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<bool> for QueryValue<'_> {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<&i32> for QueryValue<'_> {
    fn from(value: &i32) -> Self {
        Self::I32(*value)
    }
}

impl From<&bool> for QueryValue<'_> {
    fn from(value: &bool) -> Self {
        Self::Bool(*value)
    }
}

impl<'a> From<serde_json::Value> for QueryValue<'a> {
    fn from(value: serde_json::Value) -> Self {
        Self::Json(value)
    }
}

impl From<f32> for QueryValue<'_> {
    fn from(value: f32) -> Self {
        Self::OwnedStr(value.to_string())
    }
}

impl From<&f32> for QueryValue<'_> {
    fn from(value: &f32) -> Self {
        Self::OwnedStr(value.to_string())
    }
}

impl<'a> QueryValue<'a> {
    pub fn to_query_value(&self) -> Cow<'_, str> {
        match self {
            QueryValue::Str(v) => (*v).into(),
            QueryValue::I64(v) => v.to_string().into(),
            QueryValue::I32(v) => v.to_string().into(),
            QueryValue::Bool(v) => {
                if *v {
                    Cow::Borrowed("true")
                } else {
                    Cow::Borrowed("false")
                }
            }
            QueryValue::Json(v) => v.to_string().into(),
            QueryValue::OwnedStr(v) => v.as_str().into(),
        }
    }

    pub fn url_encode(&self) -> String {
        urlencoding::encode(&self.to_query_value()).into_owned()
    }
}
