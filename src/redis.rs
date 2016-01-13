use errors::RedisError;
use reader::Reader;
use results::RedisResult;
use std::fmt;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;

pub struct RedisClient {
    port: &'static str,
    host: &'static str,
    buffer: BufReader<TcpStream>
}

impl RedisClient {
    pub fn new(host: &'static str, port: &'static str) -> Result<RedisClient, RedisError> {
        TcpStream::connect(&*format!("{}:{}", host, port))
            .map(|tcp_stream| {
                    // TODO better timeout init
                    let _res_write = tcp_stream.set_write_timeout(Some(Duration::new(5, 0)));
                    let _res_read = tcp_stream.set_read_timeout(Some(Duration::new(5, 0)));
                    RedisClient {
                        port: port,
                        host: host,
                        buffer: BufReader::new(tcp_stream),
                }
            })
            .map_err(|err| RedisError::Io(err))
    }

    fn exec_command(&mut self, buf_to_send: &[u8]) -> Result<RedisResult, RedisError> {
        {
            let mut writer = self.buffer.get_mut() as &mut Write;
            try!(writer.write(buf_to_send));
        }
        
        Reader::read(&mut self.buffer)
    }

    pub fn append(&mut self, key: &str, value: &str) -> Result<i64, RedisError> {
    	let cmd = "APPEND ".to_string() + &key + &" ".to_string() + &value + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn auth(&mut self, password: &str) -> Result<String, RedisError> {
        let cmd = "AUTH ".to_string() + &password + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<String>())
    }

    pub fn bgrewriteaof(&mut self) -> Result<String, RedisError> {
        let cmd = "BGREWRITEAOF\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<String>())
    }

    pub fn bgsave(&mut self) -> Result<String, RedisError> {
        let cmd = "BGSAVE\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<String>())
    }

    pub fn bitcount(&mut self, key: &str) -> Result<i64, RedisError> {
    	let cmd = "BITCOUNT ".to_string() + &key + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn bitcount_range(&mut self, key: &str, start_range: i64, end_range: i64) -> Result<i64, RedisError> {
    	let cmd = "BITCOUNT ".to_string() + &key + &" ".to_string() + &start_range.to_string() + &" ".to_string() + &end_range.to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn del(&mut self, key: &str) -> Result<i64, RedisError> {
        let cmd = "DEL ".to_string() + &key + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn del_multi(&mut self, keys: Vec<String>) -> Result<i64, RedisError> {
        let mut cmd = "DEL".to_string();
        for key in keys {
        	cmd = cmd + &" ".to_string() + &key;
        }
        cmd = cmd + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn exists(&mut self, key: &str) -> Result<i64, RedisError> {
        let cmd = "EXISTS ".to_string() + &key + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn exists_multi(&mut self, keys: Vec<String>) -> Result<i64, RedisError> {
        let mut cmd = "EXISTS".to_string();
        for key in keys {
        	cmd = cmd + &" ".to_string() + &key;
        }
        cmd = cmd + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn expire(&mut self, key: &str, expiry: i64) -> Result<i64, RedisError> {
        let cmd = "EXPIRE ".to_string() + &key + &" ".to_string() + &expiry.to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn get<T: From<RedisResult>>(&mut self, key: &str) -> Result<T, RedisError> {
        let cmd = "GET ".to_string() + &key + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<T>())
    }

    pub fn set(&mut self, key: &str, value: &[u8]) -> Result<String, RedisError> {
        let cmd = "SET ".to_string() + &key + &" ".to_string();

        let mut cmd_bytes: Vec<u8> = cmd.into_bytes();

        cmd_bytes.extend(value.iter().cloned());
        cmd_bytes.extend([13,10].iter().cloned()); //ADD CRLF at the end of the command

        let res = try!(self.exec_command(&*cmd_bytes));      
        Ok(res.convert::<String>())
    }

    pub fn set_with_args(&mut self, key: &str, value: &[u8], args: &str) -> Result<String, RedisError> {
        let cmd = "SET ".to_string() + &key + &" ".to_string();

        let mut cmd_bytes: Vec<u8> = cmd.into_bytes();

        cmd_bytes.extend(value.iter().cloned());
        cmd_bytes.extend([32].iter().cloned()); // ADD SPACE
        cmd_bytes.extend(args.as_bytes().iter().cloned());
        cmd_bytes.extend([13,10].iter().cloned()); //ADD CRLF at the end of the command

        let res = try!(self.exec_command(&*cmd_bytes));      
        Ok(res.convert::<String>())
    }

    pub fn setex(&mut self, key: &str, value: &[u8], expiry: i64) -> Result<String, RedisError> {
        let arg = "EX ".to_string() + &expiry.to_string()[..];
        self.set_with_args(key, value, &arg[..])
    }

    pub fn ttl(&mut self, key: &str) -> Result<i64, RedisError> {
        let cmd = "TTL ".to_string() + &key + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

}

impl fmt::Debug for RedisClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Redis Client - HOST = {} : PORT + {}", self.host, self.port)
    }
}

impl fmt::Display for RedisClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Redis Client - HOST = {} : PORT + {}", self.host, self.port)
    }
}
