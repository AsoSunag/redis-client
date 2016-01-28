use std::error;
use std::error::Error;
use std::fmt;
use std::io;
use std::num;
use std::str;
use std::sync::mpsc;

#[derive(Debug, Clone)]
pub enum ParsingError {
    BadIdentifier(String),
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParsingError::BadIdentifier(ref err) => write!(f, "Invalid identifer: {}", err),
        }
    }
}

impl error::Error for ParsingError {
    fn description(&self) -> &str {
        match *self {
            ParsingError::BadIdentifier(ref err) => err,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ParsingError::BadIdentifier(ref _err) => Some(self),
        }
    }
}

#[derive(Debug)]
pub enum RedisError {
	Io(io::Error),
	Utf8(str::Utf8Error),
    ParseInt(num::ParseIntError),
    Parse(ParsingError),
    Response(String),
    MpscRecv(mpsc::RecvError),
    MpscSendBytes(mpsc::SendError<(u32, Vec<u8>)>),
    MpscTryRecv(mpsc::TryRecvError),
}

impl Clone for RedisError {
    fn clone(&self) -> RedisError {
        match *self {
            RedisError::Io(ref err) => RedisError::Io(io::Error::new(err.kind(), err.description())),
            RedisError::Utf8(ref err) => RedisError::Utf8(err.clone()),
            RedisError::ParseInt(ref err) => RedisError::ParseInt(err.clone()),
            RedisError::Parse(ref err) => RedisError::Parse(err.clone()),
            RedisError::Response(ref err) => RedisError::Response(err.clone()),
            RedisError::MpscRecv(ref err) => RedisError::MpscRecv(err.clone()),
            RedisError::MpscSendBytes(ref err) => RedisError::MpscSendBytes(err.clone()),
            RedisError::MpscTryRecv(ref err) => RedisError::MpscTryRecv(err.clone()),
        }
    }
}

impl fmt::Display for RedisError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RedisError::Io(ref err) => write!(f, "IO error: {}", err),
            RedisError::Utf8(ref err) => write!(f, "Utf8 error: {}", err),
            RedisError::ParseInt(ref err) => write!(f, "Parse Int error: {}", err),
            RedisError::Parse(ref err) => write!(f, "Parsing error: {}", err),
            RedisError::Response(ref err) => write!(f, "Response error: {}", err),
            RedisError::MpscRecv(ref err) => write!(f, "MpscRecv error: {}", err),
            RedisError::MpscSendBytes(ref err) => write!(f, "MpscSendBytes error: {}", err),
            RedisError::MpscTryRecv(ref err) => write!(f, "MpscTryRecv error: {}", err),
        }
    }
}

impl error::Error for RedisError {
    fn description(&self) -> &str {
        match *self {
            RedisError::Io(ref err) => err.description(),
            RedisError::Utf8(ref err) => err.description(),
            RedisError::ParseInt(ref err) => err.description(),
            RedisError::Parse(ref err) => err.description(),
            RedisError::Response(ref err) => err,
            RedisError::MpscRecv(ref err) => err.description(),
            RedisError::MpscSendBytes(ref err) => err.description(),
            RedisError::MpscTryRecv(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            RedisError::Io(ref err) => Some(err),
            RedisError::Utf8(ref err) => Some(err),
            RedisError::ParseInt(ref err) => Some(err),
            RedisError::Parse(ref err) => Some(err),
            RedisError::Response(ref _err) => Some(self),
            RedisError::MpscRecv(ref err) => Some(err),
            RedisError::MpscSendBytes(ref err) => Some(err),
            RedisError::MpscTryRecv(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for RedisError {
    fn from(err: io::Error) -> RedisError {
        RedisError::Io(err)
    }
}

impl From<str::Utf8Error> for RedisError {
    fn from(err: str::Utf8Error) -> RedisError {
        RedisError::Utf8(err)
    }
}

impl From<num::ParseIntError> for RedisError {
    fn from(err: num::ParseIntError) -> RedisError {
        RedisError::ParseInt(err)
    }
}

impl From<ParsingError> for RedisError {
    fn from(err: ParsingError) -> RedisError {
        RedisError::Parse(err)
    }
}

impl From<String> for RedisError {
    fn from(err: String) -> RedisError {
        RedisError::Response(err)
    }
}

impl From<mpsc::RecvError> for RedisError {
    fn from(err: mpsc::RecvError) -> RedisError {
        RedisError::MpscRecv(err)
    }
}

impl From<mpsc::SendError<(u32, Vec<u8>)>> for RedisError {
    fn from(err: mpsc::SendError<(u32, Vec<u8>)>) -> RedisError {
        RedisError::MpscSendBytes(err)
    }
}

impl From<mpsc::TryRecvError> for RedisError {
    fn from(err: mpsc::TryRecvError) -> RedisError {
        RedisError::MpscTryRecv(err)
    }
}
