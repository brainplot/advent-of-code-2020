use std::{
    ffi::OsStr,
    fs,
    io::{self, Read},
};

pub(crate) fn read_reader_to_string(mut reader: impl Read) -> io::Result<String> {
    let mut s = String::new();
    reader.read_to_string(&mut s)?;
    Ok(s)
}

pub(crate) fn stdin_into_string() -> io::Result<String> {
    read_reader_to_string(io::stdin())
}

pub(crate) fn file_into_string(path: &OsStr) -> io::Result<String> {
    fs::File::open(path).and_then(read_reader_to_string)
}
