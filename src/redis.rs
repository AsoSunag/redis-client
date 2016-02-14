extern crate rand;

use commands::RedisCommand;
use errors::RedisError;
use reader::Reader;
use results::RedisResult;
use self::rand::Rng;
use std::collections::HashMap;
use std::fmt;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc::*;
use std::time::Duration;
use std::thread;
use types::SenderType;

pub struct RedisClient {
    port: &'static str,
    host: &'static str,
    buffer: BufReader<TcpStream>,
}

pub struct RedisClientAsync {
    port: &'static str,
    host: &'static str,
    sender: Sender<(SenderType, u32, Vec<u8>)>,
    callbacks: HashMap<u32, Box<Fn(Result<RedisResult, RedisError>)>>,
    receiver: Receiver<(u32, Result<RedisResult, RedisError>)>,
    pipe_callbacks: HashMap<u32, Box<Fn(Result<Vec<RedisResult>, RedisError>)>>,
    pipe_receiver: Receiver<(u32, Result<Vec<RedisResult>, RedisError>)>
}

pub struct PubSubClientAsync {
    port: &'static str,
    host: &'static str,
    sender: Sender<(String, Vec<u8>)>
}

/// A RedisClient is a structure to send command to redis and receive the response.
/// All RedisClient's methods are performed synchronously.
/// 
/// When creating a RedisClient it will automatically create a connection. Therefore when
/// it is created it uses the host and the port.
///
/// Example:
///
/// ```
/// # fn function() -> Result<(), redis_client::errors::RedisError> {
/// let mut client = try!(redis_client::RedisClient::new("127.0.0.1", "6379"));
/// # Ok(())}
/// ```
impl RedisClient {
    pub fn new(host: &'static str, port: &'static str) -> Result<RedisClient, RedisError> {
        TcpStream::connect(&*format!("{}:{}", host, port))
            .map(|tcp_stream| {
                    // TODO better timeout init
                    let _res_write = tcp_stream.set_write_timeout(Some(Duration::new(5, 0)));
                    let _res_read = tcp_stream.set_read_timeout(Some(Duration::new(1, 0)));
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

    /// Execute a pipeline command received as an array of bytes
    fn exec_pipeline_command(&mut self, buf_to_send: &[u8], cmd_nb: usize) -> Result<Vec<RedisResult>, RedisError> {
        try!(self.write_command(buf_to_send));

        Reader::read_pipeline(&mut self.buffer, cmd_nb)
    }

    /// Execute a RedisCommand
    pub fn exec_redis_command(&mut self, redis_command: &mut RedisCommand) -> Result<RedisResult, RedisError> {
        self.exec_command(redis_command.into())
    }

    /// Execute a pipeline of RedisCommand
    pub fn exec_redis_pipeline_command(&mut self, redis_command: &mut RedisCommand) -> Result<Vec<RedisResult>, RedisError> {
        let cmd_nb: usize;
        {
            cmd_nb = redis_command.get_command_nb();
        }
        self.exec_pipeline_command(redis_command.into(), cmd_nb)
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

/// A RedisClientAsync is a structure to send command to redis and receive the response asynchronously.
/// 
/// When creating a RedisClientAsync it will automatically create a connection. Therefore when
/// it is created it uses the host and the port.
///
/// Example:
///
/// ```
/// # fn function() -> Result<(), redis_client::errors::RedisError> {
/// let mut client = try!(redis_client::RedisClientAsync::new("127.0.0.1", "6379"));
/// # Ok(())}
/// ```
impl RedisClientAsync {
    pub fn new(host: &'static str, port: &'static str) -> Result<RedisClientAsync, RedisError> {
        let (sender_tx, sender_rx) = channel::<(SenderType, u32, Vec<u8>)>();
        let (init_tx, init_rx) = channel::<Option<RedisError>>();
        let (receiver_tx, receiver_rx) = channel::<(u32, Result<RedisResult, RedisError>)>();
        let (pipe_receiver_tx, pipe_receiver_rx) = channel::<(u32, Result<Vec<RedisResult>, RedisError>)>();

        thread::spawn(move || {
            let _client = RedisClient::new(host, port)
            .map(|mut redis_client| {
                init_tx.send(None)
                .map(|_| {
                    loop {
                        match sender_rx.recv() {
                            Ok(value) => {
                                match value.0 {
                                    SenderType::Simple => {
                                        let _res = receiver_tx.send((value.1, redis_client.exec_command(&value.2[..])));
                                    },
                                    SenderType::Pipe(cmd_nb) => {
                                        let _res = pipe_receiver_tx.send((value.1, redis_client.exec_pipeline_command(&value.2[..], cmd_nb)));
                                    },
                                };
                            },
                            Err(_) => break,
                        };
                    }
                })
            })
            .map_err(|error| {
                let _res = init_tx.send(Some(error));
            });
        });

        match init_rx.recv() {
            Ok(None) => {
                Ok(RedisClientAsync {
                    port: port,
                    host: host,
                    sender: sender_tx,
                    receiver: receiver_rx,
                    callbacks: HashMap::new(),
                    pipe_receiver: pipe_receiver_rx,
                    pipe_callbacks: HashMap::new()
                })
            },
            Ok(Some(err)) =>  Err(err),
            Err(err) => Err(RedisError::MpscRecv(err)),
        }
    }

    /// Execute a redis pipeline command. The callback will be called once the command execution is over and the pump method is called.
    /// The return value indicates if the command was successfully launched.
    pub fn exec_redis_pipeline_command_async<F>(&mut self, redis_command: &mut RedisCommand, callback: F) 
        -> Result<(), RedisError> where F: Fn(Result<Vec<RedisResult>, RedisError>), F: Send + 'static
    {
        let mut rng = rand::thread_rng();
        let key = rng.gen::<u32>();
        try!(self.sender.send((SenderType::Pipe(redis_command.get_command_nb()), key, redis_command.into())));
        self.pipe_callbacks.insert(key, Box::new(callback));
        Ok(())
    }

    /// Execute a redis command. The callback will be called once the command execution is over and the pump method is called.
    /// The return value indicates if the command was successfully launched.
    pub fn exec_redis_command_async<F>(&mut self, redis_command: &mut RedisCommand, callback: F) 
        -> Result<(), RedisError> where F: Fn(Result<RedisResult, RedisError>), F: Send + 'static
    {
        let mut rng = rand::thread_rng();
        let key = rng.gen::<u32>();
        try!(self.sender.send((SenderType::Simple, key, redis_command.into())));
        self.callbacks.insert(key, Box::new(callback));
        Ok(())
    }

    /// Pump the result and execute the callbacks with them. If no result are ready this function will return.
    pub fn pump(&mut self) -> Result<(), RedisError> {
        loop {
            match self.receiver.try_recv() {
                Ok(result) => {
                    self.callbacks.remove(&result.0) 
                        .map(|callback| {
                            if result.1.is_ok() {

                                callback(result.1.clone());
                            }
                        });
                },
                Err(TryRecvError::Empty) => {
                    match self.pipe_receiver.try_recv() {
                        Ok(result) => {
                            self.pipe_callbacks.remove(&result.0) 
                                .map(|callback| {
                                    if result.1.is_ok() {

                                        callback(result.1.clone());
                                    }
                                });
                        },
                        Err(TryRecvError::Empty) => return Ok(()),
                        Err(err) => return Err(RedisError::MpscTryRecv(err))
                    };
                },
                Err(err) => return Err(RedisError::MpscTryRecv(err))
            };
        }
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


/// A PubSubClientAsync is a structure to use redis publish/subscribe functionnality.
/// 
/// When creating a PubSubClientAsync it will automatically create a connection. Therefore when
/// it is created it uses the host and the port.
///
/// Example:
///
/// ```
/// # fn function() -> Result<(), redis_client::errors::RedisError> {
/// let mut client = try!(redis_client::PubSubClientAsync::new("127.0.0.1", "6379"));
/// # Ok(())}
/// ```
impl PubSubClientAsync {
    pub fn new(host: &'static str, port: &'static str) -> Result<PubSubClientAsync, RedisError> {
        let (init_tx, init_rx) = channel::<Option<RedisError>>();
        let (sender_tx, sender_rx) = channel::<(String, Vec<u8>)>();

        thread::spawn(move || {
            let _client = RedisClient::new(host, port)
            .map(|mut redis_client| {
                init_tx.send(None)
                .map(|_| {
                    loop {
                        match sender_rx.try_recv() {
                            Ok(value) => {
                                let _res = redis_client.exec_command(&value.1[..]).unwrap();
                            },
                            Err(_) => {
                                if let Ok(res) = Reader::read(&mut redis_client.buffer) {
                                    println!("{:?}", res);
                                }
                            }
                        };
                    }
                })
            })
            .map_err(|error| {
                let _res = init_tx.send(Some(error));
            });
        });

        match init_rx.recv() {
            Ok(None) => {
                Ok(PubSubClientAsync {
                    port: port,
                    host: host,
                    sender: sender_tx
                })
            },
            Ok(Some(err)) =>  Err(err),
            Err(err) => Err(RedisError::MpscRecv(err)),
        }
    }

    /// Execute a redis command. The callback will be called once the command execution is over and the pump method is called.
    /// The return value indicates if the command was successfully launched.
    pub fn exec_redis_command_async(&mut self, redis_command: &mut RedisCommand) 
        -> Result<(), RedisError> 
    {
        try!(self.sender.send(("key".to_string(), redis_command.into())));
        Ok(())
    }
}

impl fmt::Debug for PubSubClientAsync {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Redis Client Async - HOST = {} : PORT + {}", self.host, self.port)
    }
}

impl fmt::Display for PubSubClientAsync {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Redis Client Async - HOST = {} : PORT + {}", self.host, self.port)
    }
}
