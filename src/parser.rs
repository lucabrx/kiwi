use crate::{error::DBError, res::Val};

pub fn parse_message(buf: bytes::BytesMut) -> Result<(Val, usize), DBError> {
    match buf[0] as char {
        '+' => parse_simple_string(buf),
        '$' => parse_bulk_string(buf),
        '*' => parse_array(buf),
        _ => Err(DBError::ParseError),
    }
}

fn parse_simple_string(buffer: bytes::BytesMut) -> Result<(Val, usize), DBError> {
    if let Some((buf, len)) = read_until_crlf(&buffer[1..]) {
        let string = String::from_utf8(buf.to_vec()).unwrap();
        return Ok((Val::SimpleString(string), len + 1));
    }
    Err(DBError::ParseError)
}

fn parse_array(buffer: bytes::BytesMut) -> Result<(Val, usize), DBError> {
    let (array_length, mut bytes_consumed) = if let Some((buf, len)) = read_until_crlf(&buffer[1..])
    {
        if let Ok(array_length) = parse_int(buf) {
            (array_length, len + 1)
        } else {
            return Err(DBError::ParseError);
        }
    } else {
        return Err(DBError::ParseError);
    };

    let mut array = vec![];

    for _ in 0..array_length {
        if bytes_consumed >= buffer.len() {
            return Err(DBError::ParseError);
        }
        let (item, len) = parse_message(bytes::BytesMut::from(&buffer[bytes_consumed..]))?;
        array.push(item);
        bytes_consumed += len;
    }

    Ok((Val::Arr(array), bytes_consumed))
}

fn read_until_crlf(buffer: &[u8]) -> Option<(&[u8], usize)> {
    for i in 1..buffer.len() {
        if buffer[i - 1] == b'\r' && buffer[i] == b'\n' {
            return Some((&buffer[0..(i - 1)], i + 1));
        }
    }
    None
}

fn parse_bulk_string(buffer: bytes::BytesMut) -> Result<(Val, usize), DBError> {
    let (bulk_str_length, bytes_consumed) = if let Some((buf, len)) = read_until_crlf(&buffer[1..])
    {
        if let Ok(bulk_str_length) = parse_int(buf) {
            (bulk_str_length, len + 1)
        } else {
            return Err(DBError::ParseError);
        }
    } else {
        return Err(DBError::ParseError);
    };

    let end_of_bulk_str = bytes_consumed + bulk_str_length as usize;
    let total_parsed = end_of_bulk_str + 2;

    if end_of_bulk_str > buffer.len() {
        return Err(DBError::ParseError);
    }

    if let Ok(string) = String::from_utf8(buffer[bytes_consumed..end_of_bulk_str].to_vec()) {
        return Ok((Val::BulkString(string), total_parsed));
    }

    Err(DBError::ParseError)
}

fn parse_int(buffer: &[u8]) -> Result<i64, std::num::ParseIntError> {
    String::from_utf8(buffer.to_vec()).unwrap().parse::<i64>()
}

fn unpack_bulk_string(value: Val) -> Result<String, String> {
    match value {
        Val::BulkString(s) => Ok(s),
        _ => Err(format!("Invalid bulk string {:?}", value)),
    }
}

pub fn parse_command(value: Val) -> Result<(String, Vec<String>), DBError> {
    match value {
        Val::Arr(arr) => match unpack_bulk_string(arr.first().unwrap().clone()) {
            Ok(command) => {
                let args: Result<Vec<String>, String> = arr
                    .into_iter()
                    .skip(1)
                    .map(|v| unpack_bulk_string(v.clone()))
                    .collect();

                args.map(|args| (command, args))
                    .map_err(|_| DBError::ParseError)
            }
            Err(_) => Err(DBError::ParseError),
        },
        _ => Err(DBError::ParseError),
    }
}
