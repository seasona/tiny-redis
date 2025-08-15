use std::io::{self, Cursor};

use bytes::{BytesMut};
use tklog::{info, error, debug};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;

use crate::frame::{self, Frame};

#[derive(Debug)]
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
            Ok(_) => {
                debug!("frame check ok");

                // The check function has advanced the cursor to the end of the frame.
                let len = buf.position() as usize;

                buf.set_position(0);

                let frame = Frame::parse(&mut buf)?;

                debug!("frame is {:?}", frame);

                /* discard the parsed frame from the buffer */
                let _ = self.buffer.split_to(len);

                Ok(Some(frame))
            }
            Err(frame::Error::Incomplete) => Ok(None),
            Err(e) => {
                error!("frame check error");
                Err(e.into())
            }
        }
    }

    pub async fn write_frame(&mut self, frame: &Frame) -> io::Result<()> {
        debug!("write frame {:?}", frame);

        match frame {
            Frame::Array(val) => {
                // Encode the frame type prefix, for array it's `*`
                self.stream.write_u8(b'*').await?;

                // Encode the length of the array
                self.write_decimal(val.len() as u64).await?;

                for entry in val {
                    self.write_value(entry).await?;
                }
            }
            _ => self.write_value(frame).await?,
        }

        // Flush the stream ensure the encoded frame is written to the socket
        self.stream.flush().await
    }

    async fn write_decimal(&mut self, val: u64) -> io::Result<()> {
        // Convert the decimal to a string
        let s = val.to_string();

        self.stream.write_all(s.as_bytes()).await?;
        self.stream.write_all(b"\r\n").await?;

        Ok(())
    }

    // Write a frame literal to the stream
    async fn write_value(&mut self, frame: &Frame) -> io::Result<()> {
        match frame {
            Frame::Simple(val) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Error(val) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Integer(val) => {
                self.stream.write_u8(b':').await?;
                self.write_decimal(*val).await?;
            }
            Frame::Null => {
                self.stream.write_all(b"$-1\r\n").await?;
            }
            Frame::Bulk(val) => {
                self.stream.write_u8(b'$').await?;
                self.write_decimal(val.len() as u64).await?;
                self.stream.write_all(val).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            _ => unimplemented!(),
        }

        Ok(())
    }
}
