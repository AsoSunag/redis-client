use std::collections::HashMap;
use std::fmt;
use std::str;

#[derive(Debug)]
pub enum RedisResult {
    Array(Vec<RedisResult>),
    Bytes(Vec<u8>),
    String(String),
    Int(i64),
    Nil,
}

impl RedisResult {
    pub fn convert<T: From<RedisResult>>(self) -> T {
        self.into()
    }
}

impl fmt::Display for RedisResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RedisResult::Array(ref value) => write!(f, "{:?}", value),
            RedisResult::Bytes(ref value) => write!(f, "{:?}", value),
            RedisResult::String(ref value) => write!(f, "{:?}", value),
            RedisResult::Int(ref value) => write!(f, "{:?}", value),
            RedisResult::Nil => write!(f, "null"),
        }
    }
}

impl From<RedisResult> for String {
    fn from(result: RedisResult) -> String {
        match result {
            RedisResult::Array(_value) => "Array cannot be stringified".to_string(),
            RedisResult::Bytes(value) => str::from_utf8(&value).unwrap().to_string(),
            RedisResult::String(value) => value,
            RedisResult::Int(value) => value.to_string(),
            RedisResult::Nil => "null".to_string(),
        }
    }
}


impl From<RedisResult> for Vec<u8> {
    fn from(result: RedisResult) -> Vec<u8> {
        match result {
            RedisResult::Array(_value) => vec![],
            RedisResult::Bytes(value) => value,
            RedisResult::String(value) => value.into_bytes(),
            RedisResult::Int(value) => value.to_string().into_bytes(),
            RedisResult::Nil => vec![],
        }
    }
}

impl From<RedisResult> for Vec<String> {
    fn from(result: RedisResult) -> Vec<String> {
        match result {
            RedisResult::Array(value) => {
                let mut retval = Vec::new();
                for res in value {
                    retval.push(res.convert::<String>());
                }
                retval
            },
            RedisResult::Bytes(_value) => vec![],
            RedisResult::String(value) => vec![value],
            RedisResult::Int(value) => vec![value.to_string()],
            RedisResult::Nil => vec![],
        }
    }
}

impl From<RedisResult> for HashMap<String, String> {
    fn from(result: RedisResult) -> HashMap<String, String> {
        match result {
            RedisResult::Array(value) => {
                let mut retval = HashMap::new();
                let mut key = String::new();
                for (index, res) in value.into_iter().enumerate() {
                    if index % 2 == 0 {
                        key = res.convert::<String>();
                    } else {
                        retval.insert(key.to_string(), res.convert::<String>());
                    }
                }
                retval
            },
            _ => HashMap::new(),
        }
    }
}

impl From<RedisResult> for i64 {
    fn from(result: RedisResult) -> i64 {
        match result {
            RedisResult::Array(_value) => 0,
            RedisResult::Bytes(_value) => 0,
            RedisResult::String(_value) => 0,
            RedisResult::Int(value) => value,
            RedisResult::Nil => 0,
        }
    }
}

