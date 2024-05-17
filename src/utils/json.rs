use serde_json::{from_slice, json, Value};
use std::{fs::read, path::Path};

pub fn read_json<T: AsRef<Path>>(path: T) -> Value {
    let val = read(path).unwrap_or_default();
    match from_slice(&val) {
        Ok(value) => value,
        Err(_) => json!({}),
    }
}
