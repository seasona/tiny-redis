use std::io::Cursor;
use std::io::SeekFrom;
use std::io::prelude::*;

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

impl Frame {
    pub(crate) fn check(src: &mut Cursor<&[u8]>) -> Result<(), Error> {
        match get_u8(src)? {
            b'+' => {
                // get_line(src)?;
                Ok(())
            }
        }
    } 
}

fn get_u8(src: &mut Cursor<&[u8]>) -> Result<u8, Error> {
    src.extract(SeekFrom::Current(0))?
}

// fn get_line