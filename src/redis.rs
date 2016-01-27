use commands::RedisCommand;
use errors::RedisError;
use reader::Reader;
use results::RedisResult;
use std::fmt;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc::*;
use std::time::Duration;
use std::thread;

pub struct RedisClient {
    port: &'static str,
    host: &'static str,
    buffer: BufReader<TcpStream>,
}

pub struct RedisClientAsync {
    port: &'static str,
    host: &'static str,
    sender: Sender<Vec<u8>>,
    callback_sender: Sender<Box<Fn(Result<RedisResult, RedisError>) + Send>>,
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

    /// write a command to the stream
    fn write_command(&mut self, buf_to_send: &[u8]) -> Result<usize, RedisError> {
        let mut writer = self.buffer.get_mut() as &mut Write;
        let size = try!(writer.write(buf_to_send));
        Ok(size)
    }

    /// Execute a command received as an array of bytes
    fn exec_command(&mut self, buf_to_send: &[u8]) -> Result<RedisResult, RedisError> {
        try!(self.write_command(buf_to_send));
        
        Reader::read(&mut self.buffer)
    }

    /// Execute a RedisCommand
    pub fn exec_redis_command(&mut self, redis_command: &mut RedisCommand) -> Result<RedisResult, RedisError> {
        self.exec_command(redis_command.into())
    }

    /// Execute a pipeline of RedisCommand
    pub fn exec_redis_pipeline_command(&mut self, redis_command: &mut RedisCommand) -> Result<Vec<RedisResult>, RedisError> {
        try!(self.write_command(redis_command.into()));

        Reader::read_pipeline(&mut self.buffer, redis_command.get_command_nb())
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

impl RedisClientAsync {
    pub fn new(host: &'static str, port: &'static str) -> Result<RedisClientAsync, RedisError> {
        let (tx, rx) = channel::<Vec<u8>>();
        let (ctx, crx) = channel::<Box<Fn(Result<RedisResult, RedisError>) + Send>>();
        let (itx, irx) = channel::<Option<RedisError>>();

        thread::spawn(move || {
            let _client = RedisClient::new(host, port)
            .map(|mut redis_client| {
                itx.send(None)
                .map(|_| {
                    loop {
                        match rx.recv() {
                            Ok(value) => {
                                match crx.recv() {
                                    Ok(callback) => {
                                        callback(redis_client.exec_command(&value[..]));
                                    },
                                    Err(_) => break,
                                }
                            },
                            Err(_) => break,
                        };
                    }
                })
                
            })
            .map_err(|error| {
                let _res = itx.send(Some(error));
            });
        });

        match irx.recv() {
            Ok(None) => {
                Ok(RedisClientAsync {
                    port: port,
                    host: host,
                    sender: tx,
                    callback_sender: ctx,
                })
            },
            Ok(Some(err)) =>  Err(err),
            Err(err) => Err(RedisError::MpscRecv(err)),
        }
    }

    /// Execute a redis command and call the callback when it is done with the result
    /// The return value indicates if the command was successfully launched
    pub fn exec_redis_command_async<F>(&self, redis_command: &mut RedisCommand, callback: F) 
        -> Result<(), RedisError> where F: Fn(Result<RedisResult, RedisError>), F: Send + 'static
    {
        try!(self.sender.send(redis_command.into()));
        try!(self.callback_sender.send(Box::new(callback)));
        Ok(())
    }
}

impl fmt::Debug for RedisClientAsync {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Redis Client Async - HOST = {} : PORT + {}", self.host, self.port)
    }
}

impl fmt::Display for RedisClientAsync {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Redis Client Async - HOST = {} : PORT + {}", self.host, self.port)
    }
}
