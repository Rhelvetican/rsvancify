use serde::Serialize;
use serde_json::{from_slice, json, ser::PrettyFormatter, Serializer, Value};
use std::{
    fs::{read, DirBuilder, OpenOptions},
    io::Write,
    path::Path,
};

pub fn read_json<T: AsRef<Path>>(path: T) -> Value {
    let val = read(path).unwrap_or_default();
    match from_slice(&val) {
        Ok(value) => value,
        Err(_) => json!({}),
    }
}

pub fn write_json<T: Serialize, U: AsRef<Path>>(path: U, value: T) {
    let mut file = if !path.as_ref().exists() {
        let dir = path.as_ref().parent().unwrap();
        DirBuilder::new().recursive(true).create(dir).unwrap();
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)
            .unwrap()
    } else {
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap()
    };
    let fmt = PrettyFormatter::with_indent(b"    ");
    let mut buf = Vec::new();
    let mut ser = Serializer::with_formatter(&mut buf, fmt);
    value.serialize(&mut ser).unwrap();
    file.write_all(&buf).unwrap();
}
