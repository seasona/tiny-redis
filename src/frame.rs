use std::io::Cursor;
use std::io::prelude::*;
use std::fmt;
use std::string::FromUtf8Error;
use std::num::TryFromIntError;

use bytes::Buf;
use bytes::Bytes;

#[derive(Clone, Debug)]
pub enum Frame {
    Simple(String),
    Error(String),
    Integer(u64),
    Bulk(Bytes),
    Null,
    Array(Vec<Frame>),
}

#[derive(Debug)]
pub enum Error {
    /// Not enough data for parse a message
    Incomplete,
    /// Invalid message encoding
    Other(crate::Error),
}

impl From<String> for Error {
    fn from(src: String) -> Self {
        Error::Other(src.into())
    }
}

impl From<&str> for Error {
    fn from(src: &str) -> Self {
        Error::Other(src.into())
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_src: FromUtf8Error) -> Error {
        "protocol error; invalid frame format".into()
    }
}

impl From<TryFromIntError> for Error {
    fn from(_src: TryFromIntError) -> Error {
        "protocol error; invalid frame format".into()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Incomplete => "stread ended early".fmt(fmt),
            Error::Other(err) => err.fmt(fmt),
        }
    }
}

impl std::error::Error for Error {}

impl Frame {
    /// Return a empty array frame
    pub(crate) fn array() -> Frame {
        Frame::Array(vec![])
    }

    /// Push a `bulk` frame to array
    pub(crate) fn push_bulk(&mut self, bytes: Bytes) {
        match self {
            Frame::Array(vec) => {
                vec.push(Frame::Bulk(bytes));
            }
            _ => panic!("not a array frame"),
        }
    }

    /// Converts the frame to an "unexpected frame" error
    pub(crate) fn to_error(&self) -> crate::Error {
        format!("unexpected frame: {}", self).into()
    }

    /// Check if an entire message can be decoded from `src`
    pub(crate) fn check(src: &mut Cursor<&[u8]>) -> Result<(), Error> {
        match get_u8(src)? {
            b'+' => {   // String
                get_line(src)?;
                Ok(())
            }
            b'-' => {   // Simple
                get_line(src)?;
                Ok(())
            }
            b':' => {   // Integer
                get_line(src)?;
                Ok(())
            }
            b'$' => {   // Bulk
                // TODO: handle negative length?
                let len = get_decimal(src)?;
                skip(src, len)
            }
            b'*' => {   // Array
                let len = get_decimal(src)?;
                for _ in 0..len {
                    Frame::check(src)?;
                }

                Ok(())
            }
            actual => Err(format!("protocal error; invalid frame type byte: {}", actual).into()),
        }
    }

    pub(crate) fn parse(src: &mut Cursor<&[u8]>) -> Result<Frame, Error> {
        match get_u8(src)? {
            b'+' => {
                let line = get_line(src)?.to_vec();

                let string = String::from_utf8(line)?;

                Ok(Frame::Simple(string))
            }
            b'-' => {
                let line = get_line(src)?.to_vec();

                let string = String::from_utf8(line)?;

                Ok(Frame::Error(string))
            }
            b':' => {
                let len = get_decimal(src)?;
                Ok(Frame::Integer(len))
            }
            b'$' => {
                // TODO: handle negative len
                let len = get_decimal(src)?.try_into()?;
                let n = (len + 2) as u64;

                /* copy data without \r\n */
                let data = Bytes::copy_from_slice(&src.chunk()[..len]);

                /* skip the number of bytes + 2 (\r\n) */
                skip(src, n)?;

                Ok(Frame::Bulk(data))
            }
            b'*' => {
                let len = get_decimal(src)?.try_into()?;

                let mut out = Vec::<Frame>::with_capacity(len);

                for _ in 0..len {
                    out.push(Frame::parse(src)?);
                }

                Ok(Frame::Array(out))
            }
            _ => unimplemented!()
        }
    }
}

impl fmt::Display for Frame {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use std::str;

        match self {
            Frame::Simple(s) => s.fmt(fmt),
            Frame::Error(s) => write!(fmt, "error: {}", s),
            Frame::Integer(n) => n.fmt(fmt),
            Frame::Bulk(b) => {
                if let Ok(s) = str::from_utf8(b) {
                    s.fmt(fmt)
                } else {
                    write!(fmt, "{:?}", b)
                }
            }
            Frame::Null => write!(fmt, "null"),
            Frame::Array(parts) => {
                for (i, part) in parts.iter().enumerate() {
                    if i > 0 {
                        write!(fmt, " ")?;
                    }
                    part.fmt(fmt)?;
                }

                Ok(())
            }
        }
    }
}

fn skip(src: &mut Cursor<&[u8]>, n: u64) -> Result<(), Error> {
    src.set_position(src.position() + n);
    Ok(())
}

fn get_u8(src: &mut Cursor<&[u8]>) -> Result<u8, Error> {
    let mut buf = [0u8; 1];
    match src.read_exact(&mut buf) {
        Ok(_) => Ok(buf[0]),
        Err(_) => Err(Error::Incomplete),
    }
}

// use lifetime to promise that the slice is valid with the Cursor
fn get_line<'a>(src: &mut Cursor<&'a [u8]>) -> Result<&'a [u8], Error> {
    let start = src.position() as usize;
    let end = src.get_ref().len() - 1;

    for i in start..end {
        if src.get_ref()[i] == b'\r' && src.get_ref()[i + 1] == b'\n' {
            // We found a line, update the position and return the line slice
            src.set_position((i + 2) as u64);

            return Ok(&src.get_ref()[start..i]);
        }
    }

    Err(Error::Incomplete)
}

fn get_decimal(src: &mut Cursor<&[u8]>) -> Result<u64, Error> {
    use atoi::atoi;

    let line = get_line(src)?;

    atoi::<u64>(line).ok_or_else(|| "protocol error; invalid frame format".into())
}
