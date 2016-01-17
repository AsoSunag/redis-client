use errors::RedisError;
use reader::Reader;
use results::RedisResult;
use std::collections::HashMap;
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

    pub fn append<K, V>(&mut self, key: K, value: V) -> Result<i64, RedisError> where K: ToString, V: ToString {
    	let cmd = "APPEND ".to_string() + &key.to_string().to_string() + &" ".to_string() + &value.to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn auth<P>(&mut self, password: P) -> Result<String, RedisError> where P: ToString {
        let cmd = "AUTH ".to_string() + &password.to_string() + &"\r\n".to_string();

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

    pub fn bitcount<K>(&mut self, key: K) -> Result<i64, RedisError> where K: ToString {
    	let cmd = "BITCOUNT ".to_string() + &key.to_string().to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn bitcount_range<K>(&mut self, key: K, start_range: i64, end_range: i64) -> Result<i64, RedisError> where K: ToString {
    	let cmd = "BITCOUNT ".to_string() + &key.to_string() + &" ".to_string() + &start_range.to_string() + &" ".to_string() + &end_range.to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn del<K>(&mut self, key: K) -> Result<i64, RedisError> where K: ToString {
        let cmd = "DEL ".to_string() + &key.to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn del_multi(&mut self, keys: Vec<String>) -> Result<i64, RedisError> {
        let mut cmd = "DEL".to_string();
        for key in keys {
        	cmd = cmd + &" ".to_string() + &key.to_string();
        }
        cmd = cmd + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn exists<K>(&mut self, key: K) -> Result<i64, RedisError> where K: ToString {
        let cmd = "EXISTS ".to_string() + &key.to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn exists_multi(&mut self, keys: Vec<String>) -> Result<i64, RedisError> {
        let mut cmd = "EXISTS".to_string();
        for key in keys {
        	cmd = cmd + &" ".to_string() + &key.to_string();
        }
        cmd = cmd + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn expire<K>(&mut self, key: K, expiry: i64) -> Result<i64, RedisError> where K: ToString {
        let cmd = "EXPIRE ".to_string() + &key.to_string() + &" ".to_string() + &expiry.to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn expireat<K>(&mut self, key: K, timestamp: i64) -> Result<i64, RedisError> where K: ToString {
        let cmd = "EXPIREAT ".to_string() + &key.to_string() + &" ".to_string() + &timestamp.to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn get<K, R>(&mut self, key: K) -> Result<R, RedisError> where K: ToString, R: From<RedisResult> {
        let cmd = "GET ".to_string() + &key.to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<R>())
    }

    pub fn getrange<K, R>(&mut self, key: K, start_range: i64, end_range: i64) -> Result<R, RedisError> where K: ToString, R: From<RedisResult> {
        let cmd = "GETRANGE ".to_string() + &key.to_string()  + &" ".to_string() + &start_range.to_string() + &" ".to_string() + &end_range.to_string() +  &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<R>())
    }

    pub fn hdel<K, F>(&mut self, key: K, field: F) -> Result<i64, RedisError> where K: ToString, F: ToString {
        self.hmdel(key.to_string(), vec![field.to_string()])
    }

    pub fn hmdel<K>(&mut self, key: K, fields: Vec<String>) -> Result<i64, RedisError> where K: ToString {
        let mut cmd = "HDEL ".to_string() + &key.to_string();

        for field in fields {
            cmd = cmd + &" ".to_string() + &field;
        }
        cmd = cmd + &"\r\n".to_string();
        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn hexists<K, F>(&mut self, key: K, field: F) -> Result<i64, RedisError> where K: ToString, F: ToString {
        let cmd = "HEXISTS ".to_string() + &key.to_string() + &" ".to_string() + &field.to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn hget<K, F>(&mut self, key: K, field: F) -> Result<String, RedisError> where K: ToString, F: ToString {
        let cmd = "HGET ".to_string() + &key.to_string() + &" ".to_string() + &field.to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<String>())
    }

    pub fn hgetall<K>(&mut self, key: K) -> Result<HashMap<String, String>, RedisError> where K: ToString {
        let cmd = "HGETALL ".to_string() + &key.to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<HashMap<String, String>>())
    }

    pub fn hincrby<K, F>(&mut self, key: K, field: F, increment: i64) -> Result<i64, RedisError> where K: ToString, F: ToString {
        let cmd = "HINCRBY ".to_string() + &key.to_string() + &" ".to_string() + &field.to_string() + &" ".to_string() + &increment.to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn hincrbyfloat<K, F>(&mut self, key: K, field: F, increment: f64) -> Result<String, RedisError> where K: ToString, F: ToString {
        let cmd = "HINCRBYFLOAT ".to_string() + &key.to_string() + &" ".to_string() + &field.to_string() + &" ".to_string() + &increment.to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<String>())
    }

    pub fn hkeys<K>(&mut self, key: K) -> Result<Vec<String>, RedisError> where K: ToString {
        let cmd = "HKEYS ".to_string() + &key.to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<Vec<String>>())
    }

    pub fn hlen<K>(&mut self, key: K) -> Result<i64, RedisError> where K: ToString {
        let cmd = "HLEN ".to_string() + &key.to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn hmget<K>(&mut self, key: K, fields: Vec<String>) -> Result<Vec<String>, RedisError> where K: ToString {
        let mut cmd = "HMGET ".to_string() + &key.to_string();

        for field in fields {
            cmd = cmd + &" ".to_string() + &field.to_string();
        }
        cmd = cmd + &"\r\n".to_string();
        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<Vec<String>>())
    }

    pub fn hmset<K>(&mut self, key: K, fields: HashMap<String, String>) -> Result<String, RedisError> where K: ToString {
        let mut cmd = "HMSET ".to_string() + &key.to_string();

        for (field, value) in fields {
            cmd = cmd + &" ".to_string() + &field.to_string() + &" ".to_string() + &value;
        }
        cmd = cmd + &"\r\n".to_string();
        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<String>())
    }

    pub fn hset<K, F, V>(&mut self, key: K, field: F, value: V) -> Result<i64, RedisError> where K: ToString, F: ToString, V: ToString {
        let cmd = "HSET ".to_string() + &key.to_string() + &" ".to_string() + &field.to_string() + &" ".to_string() + &value.to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn hstrlen<K, F, V>(&mut self, key: K, field: F) -> Result<i64, RedisError> where K: ToString, F: ToString, V: ToString {
        let cmd = "HSTRLEN ".to_string() + &key.to_string() + &" ".to_string() + &field.to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn hsetnx<K, F, V>(&mut self, key: K, field: F, value: V) -> Result<i64, RedisError> where K: ToString, F: ToString, V: ToString {
        let cmd = "HSETNX ".to_string() + &key.to_string() + &" ".to_string() + &field.to_string() + &" ".to_string() + &value.to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn hvals<K>(&mut self, key: K) -> Result<Vec<String>, RedisError> where K: ToString {
        let cmd = "HVALS ".to_string() + &key.to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<Vec<String>>())
    }

    pub fn select(&mut self, db_index: u32) -> Result<String, RedisError> {
        let cmd = "SELECT ".to_string() + &db_index.to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<String>())
    }

    pub fn set<K, V>(&mut self, key: K, value: V) -> Result<String, RedisError> where K: ToString, V: ToString {
        let val: &[u8] = &*value.to_string().into_bytes();
        self.set_binary_with_args(key, val, "")
    }

    pub fn set_binary_with_args<K, A>(&mut self, key: K, value: &[u8], args: A) -> Result<String, RedisError> where K: ToString, A: ToString {
        let cmd = "SET ".to_string() + &key.to_string() + &" ".to_string();

        let mut cmd_bytes: Vec<u8> = cmd.into_bytes();

        cmd_bytes.extend(value.iter().cloned());
        if args.to_string().len() > 0 {
            cmd_bytes.extend([32].iter().cloned()); // ADD SPACE
            cmd_bytes.extend(args.to_string().into_bytes().iter().cloned());
        }
        
        cmd_bytes.extend([13,10].iter().cloned()); //ADD CRLF at the end of the command

        let res = try!(self.exec_command(&*cmd_bytes));      
        Ok(res.convert::<String>())
    }

    pub fn setex<K, V>(&mut self, key: K, value: String, expiry: i64) -> Result<String, RedisError> where K: ToString, V: ToString  {
        let args = "EX ".to_string() + &expiry.to_string();
        self.set_binary_with_args(key, &*value.to_string().into_bytes(), args)
    }

    pub fn psetex<K, V>(&mut self, key: K, value: String, expiry: i64) -> Result<String, RedisError> where K: ToString, V: ToString  {
        let args = "PX ".to_string() + &expiry.to_string();
        self.set_binary_with_args(key, &*value.to_string().into_bytes(), args)
    }

    pub fn setnx<K, V>(&mut self, key: K, value: String) -> Result<String, RedisError> where K: ToString, V: ToString {
        let args = "NX".to_string();
        self.set_binary_with_args(key, &*value.to_string().into_bytes(), args)
    }

    pub fn setxx<K, V>(&mut self, key: K, value: String) -> Result<String, RedisError> where K: ToString, V: ToString {
        let args = "XX".to_string();
        self.set_binary_with_args(key, &*value.to_string().into_bytes(), args)
    }

    pub fn setex_nx<K, V>(&mut self, key: K, value: String, expiry: i64) -> Result<String, RedisError> where K: ToString, V: ToString {
        let args = "EX ".to_string() + &expiry.to_string() + &" NX".to_string();
        self.set_binary_with_args(key, &*value.into_bytes(), args)
    }

    pub fn setex_xx<K, V>(&mut self, key: K, value: String, expiry: i64) -> Result<String, RedisError> where K: ToString, V: ToString {
        let args = "EX ".to_string() + &expiry.to_string() + &" XX".to_string();
        self.set_binary_with_args(key, &*value.to_string().into_bytes(), args)
    }

    pub fn psetex_nx<K, V>(&mut self, key: K, value: String, expiry: i64) -> Result<String, RedisError> where K: ToString, V: ToString {
        let args = "PX ".to_string() + &expiry.to_string() + &" NX".to_string();
        self.set_binary_with_args(key, &*value.to_string().into_bytes(), args)
    }

    pub fn psetex_xx<K, V>(&mut self, key: K, value: String, expiry: i64) -> Result<String, RedisError> where K: ToString, V: ToString {
        let args = "PX ".to_string() + &expiry.to_string() + &" XX".to_string();
        self.set_binary_with_args(key, &*value.to_string().into_bytes(), args)
    }

    pub fn ttl<K>(&mut self, key: K) -> Result<i64, RedisError> where K: ToString {
        let cmd = "TTL ".to_string() + &key.to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn zadd<K, V>(&mut self, key: K, score: f64, member: V) -> Result<i64, RedisError> where K: ToString, V: ToString {
        self.zadd_binary_with_args(key, score, &member.to_string().into_bytes()[..], "")
    }

    pub fn zadd_binary_with_args<K, A>(&mut self, key: K, score: f64, member: &[u8], args: A) -> Result<i64, RedisError> where K: ToString, A: ToString {
        let cmd = "ZADD ".to_string() + &key.to_string() + &" ".to_string() + &args.to_string() + &" ".to_string() + &score.to_string() + &" ".to_string();

        let mut cmd_bytes: Vec<u8> = cmd.into_bytes();

        cmd_bytes.extend(member.iter().cloned());
        cmd_bytes.extend([13,10].iter().cloned()); //ADD CRLF at the end of the command

        let res = try!(self.exec_command(&*cmd_bytes));      
        Ok(res.convert::<i64>())
    }

    pub fn zaddnx<K, V>(&mut self, score: f64, key: K, member: V) -> Result<i64, RedisError> where K: ToString, V: ToString {
        let args = "NX".to_string();
        self.zadd_binary_with_args(key, score, &member.to_string().into_bytes()[..], args)
    }

    pub fn zaddxx<K, V>(&mut self, score: f64, key: K, member: V) -> Result<i64, RedisError> where K: ToString, V: ToString {
        let args = "XX".to_string();
        self.zadd_binary_with_args(key, score, &member.to_string().into_bytes()[..], args)
    }

    pub fn zaddnx_ch<K, V>(&mut self, score: f64, key: K, member: V) -> Result<i64, RedisError> where K: ToString, V: ToString {
        let args = "NX ".to_string() + &" CH".to_string();
        self.zadd_binary_with_args(key, score, &member.to_string().into_bytes()[..], args)
    }

    pub fn zaddxx_ch<K, V>(&mut self, score: f64, key: K, member: V) -> Result<i64, RedisError> where K: ToString, V: ToString {
        let args = "XX ".to_string() + &" CH".to_string();
        self.zadd_binary_with_args(key, score, &member.to_string().into_bytes()[..], args)
    }

    pub fn zcard<K>(&mut self, key: K) -> Result<i64, RedisError> where K: ToString {
        let cmd = "ZCARD ".to_string() + &key.to_string()  + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn zcount<K, S, E>(&mut self, key: K, start_range: S, end_range: E) -> Result<i64, RedisError> where K: ToString, S: ToString, E: ToString  {
        let cmd = "ZCOUNT ".to_string() + &key.to_string()  + &" ".to_string() + &start_range.to_string() + &" ".to_string() + &end_range.to_string() +  &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<i64>())
    }

    pub fn zincrby<K, V>(&mut self, key: K, increment: f64, member: V) -> Result<String, RedisError> where K: ToString, V: ToString {
        let cmd = "ZINCRBY ".to_string() + &key.to_string()  + &" ".to_string() + &increment.to_string()+ &" ".to_string() + &member.to_string() + &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<String>())
    }

    pub fn zrange<K, S, E>(&mut self, key: K, start_range: S, end_range: E) -> Result<Vec<String>, RedisError> where K: ToString, S: ToString, E: ToString {
        let cmd = "ZRANGE ".to_string() + &key.to_string()  + &" ".to_string() + &start_range.to_string() + &" ".to_string() + &end_range.to_string() +  &"\r\n".to_string();

        let res = try!(self.exec_command(cmd.as_bytes()));      
        Ok(res.convert::<Vec<String>>())
    }

    pub fn zrange_with_scores<K, S, E>(&mut self, key: K, start_range: S, end_range: E) -> Result<Vec<String>, RedisError> where K: ToString, S: ToString, E: ToString {
        let cmd = "ZRANGE ".to_string() + &key.to_string()  + &" ".to_string() + &start_range.to_string() + &" ".to_string() + &end_range.to_string() +  &" WITHSCORES\r\n".to_string();

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
