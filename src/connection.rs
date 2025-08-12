use std::io::Cursor;

use bytes::{Buf, BytesMut};
use tokio::io::{AsyncReadExt, BufWriter};
use tokio::net::TcpStream;

use crate::frame::Frame;

pub struct Connection {
    // tokio buffer provide write buffer, can speed up writing
    stream: BufWriter<TcpStream>,

    // for reading frame
    buffer: BytesMut,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(socket),
            // default 4KB
            buffer: BytesMut::with_capacity(4 * 1024),
        }
    }

    /// Read from socket and fullfill a frame
    pub async fn read_frame(&mut self) -> crate::Result<Option<Frame>> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    fn parse_frame(&mut self) -> crate::Result<Option<Frame>> {
        /* create a slice for buffer */
        let mut buf = Cursor::new(&self.buffer[..]);

        match Frame::check(&mut buf) {
            
        }
    }
}
