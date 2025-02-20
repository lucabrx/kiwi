use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{error::DBError, parser::parse_message};

#[derive(Clone, Debug)]
pub enum Val {
    NullString(),
    NullArr(),
    SimpleString(String),
    BulkString(String),
    Arr(Vec<Val>),
    Error(String),
    Int(i64),
}

impl Val {
    pub fn serialize(self) -> String {
        match self {
            Val::NullString() => "$-1\r\n".to_string(),
            Val::NullArr() => "*-1\r\n".to_string(),
            Val::SimpleString(s) => format!("+{}\r\n", s),
            Val::BulkString(s) => format!("${}\r\n{}\r\n", s.len(), s),
            Val::Arr(v) => {
                let mut res = format!("*{}\r\n", v.len());
                for val in v {
                    res.push_str(&val.serialize());
                }
                res
            }
            Val::Error(s) => format!("-{}\r\n", s),
            Val::Int(i) => format!(":{}\r\n", i),
        }
    }
}

pub struct ResHandler {
    buf: bytes::BytesMut,
    stream: tokio::net::TcpStream,
}

impl ResHandler {
    pub fn new(stream: tokio::net::TcpStream) -> Self {
        Self {
            buf: bytes::BytesMut::new(),
            stream,
        }
    }

    pub async fn read_value(&mut self) -> Result<Option<Val>, DBError> {
        match self.stream.read_buf(&mut self.buf).await {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    return Ok(None);
                }
                match parse_message(self.buf.split()) {
                    Ok((val, _)) => Ok(Some(val)),
                    Err(_) => Err(DBError::ParseError),
                }
            }
            Err(_) => Err(DBError::Internal),
        }
    }

    pub async fn write_value(&mut self, response: Val) -> Result<(), DBError> {
        let _ = self.stream.write(response.serialize().as_bytes()).await;
        Ok(())
    }
}
