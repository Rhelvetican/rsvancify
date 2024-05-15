use serde_json::{from_slice, json, Value};
use std::{
    fs::{read, DirBuilder},
    path::PathBuf,
};

pub fn read_json(path: &str) -> Value {
    let val = match read(path) {
        Ok(file) => file,
        Err(_) => {
            let dir = PathBuf::from(path);
            let dir = dir.parent().unwrap();
            DirBuilder::new().recursive(true).create(dir).unwrap();
            return json!({});
        }
    };
    match from_slice(&val) {
        Ok(value) => value,
        Err(_) => json!({}),
    }
}
