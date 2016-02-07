//! A redis client implemented in Rust. It's create is called "redis-client".
//! 
//! # Connection
//!
//! Every clients are created with a host and a port:
//! 
//! ```plain
//! try!(client::new("127.0.0.1", "6379"));
//! ```
//! They are trying to connect when they are created and the new method return a Result with either the client or a RedisError.
//!
//! # The clients
//! 
//! There is more than one client in the library.
//!
//! ## RedisClient
//! 
//! It is used to execute redis commands synchronously.
//!
//! For example:
//!
//! ```no_run
//! # use redis_client::commands::CommandSender;
//! # fn function() -> Result<(), redis_client::errors::RedisError> {
//! # let mut client = try!(redis_client::RedisClient::new("127.0.0.1", "6379"));
//! let result: String = try!(client.set("key", "value"));
//! # Ok(())}
//! ```
//!
//! ## RedisClientAsync
//!
//! It is used to execute redis commands synchronously.
//!
//! For example:
//!
//! ```no_run
//! # use redis_client::commands::CommandSenderAsync;
//! # fn function() -> Result<(), redis_client::errors::RedisError> {
//! # let mut async_client = try!(redis_client::RedisClientAsync::new("127.0.0.1", "6379"));
//! try!(async_client.get("key", |result| {
//!    let result_value: String  = match result {
//!        Ok(value) => value.into(),
//!        Err(err) => err.to_string(),
//!    };
//!    println!("{:?}", result_value);
//! }));
//! # Ok(())}
//! ```
//! To get the callback to be called once the command execution is over, the pump method needs to be called.
//!
//! # Commands
//!	 
//! ## Built-in Commands
//! 
//! They are the redis commands implemented in the traits CommandBuilder to build a RedisCommand, or in CommandSender and CommandSenderAsync to send them.
//! 
//! For example:
//!
//! ```no_run
//! # use redis_client::commands::CommandBuilder;
//! # fn function() -> Result<(), redis_client::errors::RedisError> {
//! # let mut client = try!(redis_client::RedisClient::new("127.0.0.1", "6379"));
//! # let mut async_client = try!(redis_client::RedisClientAsync::new("127.0.0.1", "6379"));
//! let cmd = &mut redis_client::RedisCommand::new();
//! cmd.get("key");
//! 
//! let result: String = try!(client.exec_redis_command(cmd)).into();
//! 
//! try!(async_client.exec_redis_command_async(cmd, |result| {
//!    let result_value: String  = match result {
//!        Ok(value) => value.into(),
//!        Err(err) => err.to_string(),
//!    };
//!    println!("{:?}", result_value);
//! }));
//! # Ok(())}
//! ```
//!
//! Is the same as:
//!
//! ```no_run
//! # use redis_client::commands::{CommandSender, CommandSenderAsync};
//! # fn function() -> Result<(), redis_client::errors::RedisError> {
//! # let mut client = try!(redis_client::RedisClient::new("127.0.0.1", "6379"));
//! # let mut async_client = try!(redis_client::RedisClientAsync::new("127.0.0.1", "6379"));
//! let result: String = try!(client.get("key"));
//! 
//! try!(async_client.get("key", |result| {
//!    let result_value: String  = match result {
//!        Ok(value) => value.into(),
//!        Err(err) => err.to_string(),
//!    };
//!    println!("{:?}", result_value);
//! }));
//! # Ok(())}
//! ```
//! Some redis commands can have an argument with one or more values. For these ones the rules is to implement two built-in commands, 
//! one for the single value case that have the name of the redis commands and another for the multiple values case with the same name but 
//! prefixed by a m.
//! 
//! For example:
//!
//! ```no_run
//! # use redis_client::commands::CommandSender;
//! # fn function() -> Result<(), redis_client::errors::RedisError> {
//! # let mut client = try!(redis_client::RedisClient::new("127.0.0.1", "6379"));
//! let result: String = try!(client.del("key"));
//! 
//! let mresult: String = try!(client.mdel(vec!["key1", "key2"]));
//! # Ok(())}
//! ```
//! Another rule is when a redis commands has behavioral arguments, extra built-in commands are created.
//! For example:
//!
//! ```no_run
//! # use redis_client::commands::CommandSender;
//! # fn function() -> Result<(), redis_client::errors::RedisError> {
//! # let mut client = try!(redis_client::RedisClient::new("127.0.0.1", "6379"));
//! let result: String = try!(client.setxx("key", "value")); // SET command with the XX argument
//! # Ok(())}
//! ```
//! In the case a behavioral argument is mandatory, only the built-in commands with the arguments are created.
//! For example lindex has an argument that is either AFTER or BEFORE:
//!
//! ```no_run
//! # use redis_client::commands::CommandSender;
//! # fn function() -> Result<(), redis_client::errors::RedisError> {
//! # let mut client = try!(redis_client::RedisClient::new("127.0.0.1", "6379"));
//! let aresult: String = try!(client.linsert_after("key", "pivot", "value")); // LINSERT with AFTER has an argument
//! let bresult: String = try!(client.linsert_before("key", "pivot", "value")); // LINSERT  with BEFORE has an argument
//! // let eresult: String = try!(client.linsert("key", "pivot", "value")); DOES NOT EXIST
//! # Ok(())}
//! ```
//! ## Custom Commands
//! 
//! It is also possible to build custom command and execute them.
//! 
//! For example:
//!
//! ```no_run
//! # use redis_client::commands::CommandBuilder;
//! # fn function() -> Result<(), redis_client::errors::RedisError> {
//! # let mut client = try!(redis_client::RedisClient::new("127.0.0.1", "6379"));
//! # let mut async_client = try!(redis_client::RedisClientAsync::new("127.0.0.1", "6379"));
//! let cmd = &mut redis_client::RedisCommand::new();
//! cmd.add_cmd("GET").add_arg("key").end();
//! 
//! let result: String = try!(client.exec_redis_command(cmd)).into();
//! 
//! try!(async_client.exec_redis_command_async(cmd, |result| {
//!    let result_value: String  = match result {
//!        Ok(value) => value.into(),
//!        Err(err) => err.to_string(),
//!    };
//!    println!("{:?}", result_value);
//! }));
//! # Ok(())}
//! ```
//! # Pipeline
//!
//! To execute a command pipeline you simply need to create a RedisCommand with one or more redis commands and execute it with a client's pipeline method.
//! The only difference with the execution of a single command is that the Result contains a vector of RedisResult instead of just a RedisResult.
//!
//! Example:
//!
//! ```no_run
//! # use redis_client::commands::CommandBuilder;
//! # fn function() -> Result<(), redis_client::errors::RedisError> {
//! # let mut client = try!(redis_client::RedisClient::new("127.0.0.1", "6379"));
//! # let mut async_client = try!(redis_client::RedisClientAsync::new("127.0.0.1", "6379"));
//! let cmd = &mut redis_client::RedisCommand::new();
//! cmd.set("key", "value2").get("key");
//! 
//! let results = try!(client.exec_redis_pipeline_command(cmd)); // results is a Vec<RedisResult>
//! 
//! try!(async_client.exec_redis_pipeline_command_async(cmd, |results| {
//! 	match results {
//!  		Ok(values) => {
//!   			for value in values {
//!      			println!("{:?}", value.convert::<String>())
//!     		}
//!  		},
//!     	Err(err) => println!("{:?}", err.to_string()),
//!   	};
//! }));
//! # Ok(())}
//! ```
//! 
//! # Redis Transaction
//! The transcation commands are part of the built-in commands and therefore can be used like any other commmands.

pub use errors::{ParsingError, RedisError};
pub use redis::{RedisClient, RedisClientAsync};
pub use results::RedisResult;
pub use commands::{CommandBuilder, CommandSender, CommandSenderAsync, RedisCommand};

pub mod commands;
pub mod errors;
pub mod reader;
pub mod redis;
pub mod results;
pub mod types;
