use std::io::{BufReader, BufWriter, Read, Write};

use crate::prelude::SessionError;

pub type UID = u64;
pub type SessionResult<T> = std::result::Result<T, SessionError>;

pub enum Event {}

pub enum Stream {
    File(
        std::fs::File,
        BufWriter<std::fs::File>,
        BufReader<std::fs::File>,
    ),
    None,
}

impl Write for Stream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Stream::File(_, writer, _) => writer.write(buf),
            Stream::None => Ok(0),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Stream::File(_, writer, _) => writer.flush(),
            Stream::None => Ok(()),
        }
    }
}

impl Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Stream::File(_, _, reader) => reader.read(buf),
            Stream::None => Ok(0),
        }
    }
}
