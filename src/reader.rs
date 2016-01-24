use errors::ParsingError;
use errors::RedisError;
use results::RedisResult;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::TcpStream;

pub struct Reader;

impl Reader {
    /// Read the stream expecting one response.
    /// Determine the type of the response
    pub fn read(buffer: &mut BufReader<TcpStream>) -> Result<RedisResult, RedisError> {
        
        let mut head_line = String::new();
        try!(buffer.read_line(&mut head_line));

        let identifier = head_line.remove(0);

        match identifier{
                '$' => Reader::read_bulk_string(&head_line, buffer),
                '*' => Reader::read_array(&head_line, buffer),
                '+' => Reader::read_string(&head_line),
                ':' => Reader::read_integer(&head_line),
                '-' => Reader::read_error(&head_line),
                _ => Err(RedisError::Parse(ParsingError::BadIdentifier(identifier.to_string()))),
            }
    }

    /// Read the stream and expect several responses
    pub fn read_pipeline(buffer: &mut BufReader<TcpStream>, cmd_nb: usize) -> Result<Vec<RedisResult>, RedisError> {
        let mut results: Vec<RedisResult> = Vec::with_capacity(cmd_nb);
        let mut remaining_cmd = cmd_nb;
        loop {
            if remaining_cmd == 0 {
                break;
            }
            remaining_cmd -= 1;

            match Reader::read(buffer) {
                Ok(value) => results.push(value),
                Err(RedisError::Response(err)) => results.push(RedisResult::String(err)),
                Err(err) => return Err(err),
            };
        }
        Ok(results)
    }

    /// Read a bulk string response
    fn read_bulk_string(head_line: & String, buffer: &mut BufReader<TcpStream>) -> Result<RedisResult, RedisError> {
        let read_byte_nb: i64 = try!(head_line.trim().parse());

        if read_byte_nb < 0 {
            Ok(RedisResult::Nil)
        } else {
            let mut result: Vec<u8> = Vec::with_capacity((read_byte_nb + 2) as usize);
            loop {
                let length = {
                    let buf = try!(buffer.fill_buf());
                    result.extend(buf.iter().cloned());

                    buf.len()
                };
                

                if result.len() >= (read_byte_nb + 2) as usize {
                    buffer.consume(length - (result.len() - (read_byte_nb + 2) as usize));
                    break;
                } else {
                    buffer.consume(length);
                }
            }
            result.truncate(read_byte_nb as usize);

            Ok(RedisResult::Bytes(result))
        }
    }

    /// Read a simple string response
    fn read_string(simple_str: & String) -> Result<RedisResult, RedisError> {
        Ok(RedisResult::String(simple_str.trim().to_string()))
    }

    /// Read an integer response
    fn read_integer(integer_str: & String) -> Result<RedisResult, RedisError> {
        Ok(RedisResult::Int(try!(integer_str.trim().parse::<i64>())))
    }

    /// Read an error response
    fn read_error(error_str: & String) -> Result<RedisResult, RedisError> {
        Err(RedisError::Response(error_str.to_string()))
    }

    /// Read an array response
    fn read_array(array_str: & String, buffer: &mut BufReader<TcpStream>) -> Result<RedisResult, RedisError> {
        let mut read_elmt_nb: i64 = try!(array_str.trim().parse());

        if read_elmt_nb < 0 {
            Ok(RedisResult::Nil)
        } else if read_elmt_nb == 0 {
            Ok(RedisResult::Array(Vec::new()))
        }else {
            let mut result: Vec<RedisResult> = Vec::with_capacity(read_elmt_nb as usize);

            loop {
                match Reader::read(buffer) {
                    Ok(value) => result.push(value),
                    Err(RedisError::Response(err)) => result.push(RedisResult::String(err)),
                    Err(err) => return Err(err),
                };

                read_elmt_nb -= 1;
                if read_elmt_nb == 0 {
                    break;
                }
            }
            Ok(RedisResult::Array(result))
        }
    }

}