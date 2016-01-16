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

    pub fn expireat(&mut self, key: &str, timestamp: i64) -> Result<i64, RedisError> {
        let cmd = "EXPIREAT ".to_string() + &key + &" ".to_string() + &timestamp.to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn get<T: From<RedisResult>>(&mut self, key: &str) -> Result<T, RedisError> {
        let cmd = "GET ".to_string() + &key + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<T>())
    }

    pub fn getrange<T: From<RedisResult>>(&mut self, key: &str, start_range: i64, end_range: i64) -> Result<T, RedisError> {
        let cmd = "GETRANGE ".to_string() + &key  + &" ".to_string() + &start_range.to_string() + &" ".to_string() + &end_range.to_string() +  &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<T>())
    }


    pub fn set(&mut self, key: &str, value: &[u8]) -> Result<String, RedisError> {
        let cmd = "SET ".to_string() + &key + &" ".to_string();

        let mut cmd_bytes: Vec<u8> = cmd.into_bytes();

        cmd_bytes.extend(value.iter().cloned());

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

    pub fn psetex(&mut self, key: &str, value: &[u8], expiry: i64) -> Result<String, RedisError> {
        let arg = "PX ".to_string() + &expiry.to_string()[..];
        self.set_with_args(key, value, &arg[..])
    }

    pub fn setnx(&mut self, key: &str, value: &[u8]) -> Result<String, RedisError> {
        let arg = "NX".to_string();
        self.set_with_args(key, value, &arg[..])
    }

    pub fn setxx(&mut self, key: &str, value: &[u8]) -> Result<String, RedisError> {
        let arg = "XX".to_string();
        self.set_with_args(key, value, &arg[..])
    }

    pub fn setex_nx(&mut self, key: &str, value: &[u8], expiry: i64) -> Result<String, RedisError> {
        let arg = "EX ".to_string() + &expiry.to_string()[..] + &" NX".to_string();
        self.set_with_args(key, value, &arg[..])
    }

    pub fn setex_xx(&mut self, key: &str, value: &[u8], expiry: i64) -> Result<String, RedisError> {
        let arg = "EX ".to_string() + &expiry.to_string()[..] + &" XX".to_string();
        self.set_with_args(key, value, &arg[..])
    }

    pub fn psetex_nx(&mut self, key: &str, value: &[u8], expiry: i64) -> Result<String, RedisError> {
        let arg = "PX ".to_string() + &expiry.to_string()[..] + &" NX".to_string();
        self.set_with_args(key, value, &arg[..])
    }

    pub fn psetex_xx(&mut self, key: &str, value: &[u8], expiry: i64) -> Result<String, RedisError> {
        let arg = "PX ".to_string() + &expiry.to_string()[..] + &" XX".to_string();
        self.set_with_args(key, value, &arg[..])
    }

    pub fn ttl(&mut self, key: &str) -> Result<i64, RedisError> {
        let cmd = "TTL ".to_string() + &key + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn zadd(&mut self, key: &str, score: f64, member: &[u8]) -> Result<i64, RedisError> {
        let cmd = "ZADD ".to_string() + &key + &" ".to_string() + &score.to_string() + &" ".to_string();

        let mut cmd_bytes: Vec<u8> = cmd.into_bytes();

        cmd_bytes.extend(member.iter().cloned());

        let res = try!(self.exec_command(&*cmd_bytes));      
        Ok(res.convert::<i64>())
    }

    pub fn zadd_with_args(&mut self, key: &str, score: f64, member: &[u8], args: &str) -> Result<i64, RedisError> {
        let cmd = "ZADD ".to_string() + &key + &" ".to_string() + &args + &" ".to_string() + &score.to_string() + &" ".to_string();

        let mut cmd_bytes: Vec<u8> = cmd.into_bytes();

        cmd_bytes.extend(member.iter().cloned());
        cmd_bytes.extend([13,10].iter().cloned()); //ADD CRLF at the end of the command

        let res = try!(self.exec_command(&*cmd_bytes));      
        Ok(res.convert::<i64>())
    }

    pub fn zaddnx(&mut self, score: f64, key: &str, member: &[u8]) -> Result<i64, RedisError> {
        let arg = "NX".to_string();
        self.zadd_with_args(key, score, member, &arg[..])
    }

    pub fn zaddxx(&mut self, score: f64, key: &str, member: &[u8]) -> Result<i64, RedisError> {
        let arg = "XX".to_string();
        self.zadd_with_args(key, score, member, &arg[..])
    }

    pub fn zaddnx_ch(&mut self, score: f64, key: &str, member: &[u8]) -> Result<i64, RedisError> {
        let arg = "NX ".to_string() + &" CH".to_string();
        self.zadd_with_args(key, score, member, &arg[..])
    }

    pub fn zaddxx_ch(&mut self, score: f64, key: &str, member: &[u8]) -> Result<i64, RedisError> {
        let arg = "XX ".to_string() + &" CH".to_string();
        self.zadd_with_args(key, score, member, &arg[..])
    }

    pub fn zcard(&mut self, key: &str) -> Result<i64, RedisError> {
        let cmd = "ZCARD ".to_string() + &key  + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn zcount(&mut self, key: &str, start_range: &str, end_range: &str) -> Result<i64, RedisError> {
        let cmd = "ZCOUNT ".to_string() + &key  + &" ".to_string() + &start_range.to_string() + &" ".to_string() + &end_range.to_string() +  &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn zincrby(&mut self, key: &str, increment: f64, member: &[u8]) -> Result<String, RedisError> {
        let cmd = "ZINCRBY ".to_string() + &key  + &" ".to_string() + &increment.to_string()+ &" ".to_string();

        let mut cmd_bytes: Vec<u8> = cmd.into_bytes();

        cmd_bytes.extend(member.iter().cloned());
        cmd_bytes.extend([13,10].iter().cloned()); //ADD CRLF at the end of the command

        let res = try!(self.exec_command(&*cmd_bytes));      
        Ok(res.convert::<String>())
    }

    pub fn zrange(&mut self, key: &str, start_range: i64, end_range: i64) -> Result<Vec<String>, RedisError> {
        let cmd = "ZRANGE ".to_string() + &key  + &" ".to_string() + &start_range.to_string() + &" ".to_string() + &end_range.to_string() +  &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<Vec<String>>())
    }

    pub fn zrange_with_scores(&mut self, key: &str, start_range: i64, end_range: i64) -> Result<Vec<String>, RedisError> {
        let cmd = "ZRANGE ".to_string() + &key  + &" ".to_string() + &start_range.to_string() + &" ".to_string() + &end_range.to_string() +  &" WITHSCORES\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));    
        Ok(res.convert::<Vec<String>>())
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
