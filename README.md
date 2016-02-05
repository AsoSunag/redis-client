[![Build Status](https://travis-ci.org/AsoSunag/redis-client.svg?branch=master)](https://travis-ci.org/AsoSunag/redis-client)
[![Crates.io](https://img.shields.io/crates/v/redis-client.svg)](https://crates.io/crates/redis-client)

# redis-client
Redis client in Rust

[Documentation](https://asosunag.github.io/redis-client/redis_client/)
 
# Getting started

The crate is called "redis-client".

# API examples

## set and get

``` rust
extern crate redis_client;

use redis_client::commands::CommandSender;
use redis_client::errors::RedisError;

fn set_and_get() -> Result<String, RedisError> {
    let mut client = try!(redis_client::RedisClient::new("localhost", "6379"));
    let set_result: String = try!(client.set("key", "value"));
    let get_result: String = try!(client.get("key"));
    Ok(get_result)
}

```
## Async get

``` rust
extern crate redis_client;

use redis_client::commands::CommandSenderAsync;
use redis_client::errors::RedisError;

fn async_get() -> Result<(), RedisError> {
    let mut async_client = try!(redis_client::RedisClientAsync::new("localhost", "6379"));
    // the result argument in the closure is a Result<RedisResult, RedisError>
    try!(async_client.get("key", |result| {
        let result_value: String  = match result {
            Ok(value) => value.into(),
            Err(err) => err.to_string(),
        };
        println!("{:?}", result_value);
    }));

     try!(async_client.get("key2", |result| {
        let result_value: String  = match result {
            Ok(value) => value.into(),
            Err(err) => err.to_string(),
        };
        println!("{:?}", result_value);
    }));

    loop {

        sleep(Duration::new(0, 1000 * 1000 * 1000));

        // this method will call callbacks when their command executions are over.
        async_client.pump();
    }

    Ok(())
}

```

## Pipeline

``` rust
extern crate redis_client;

use redis_client::commands::{CommandBuilder, CommandSender, CommandSenderAsync};
use redis_client::errors::RedisError;

fn pipeline() -> Result<Vec<RedisResult>, RedisError> {
    let mut client = try!(redis_client::RedisClient::new("localhost", "6379"));
    let cmd = &mut redis_client::RedisCommand::new();
    cmd.set("key", "value2").get("key");
    let results = try!(client.exec_redis_pipeline_command(cmd));
    Ok(results)
}

fn async_pipeline() -> Result<(), RedisError> {
    let mut async_client = try!(redis_client::RedisClientAsync::new("localhost", "6379"));
    let cmd = &mut redis_client::RedisCommand::new();
    cmd.set("key", "value2").get("key");
    try!(async_client.exec_redis_pipeline_command_async(cmd, |results| {
        match results {
            Ok(values) => {
                for value in values {
                    println!("{:?}", value.convert::<String>())
                }
            },
            Err(err) => println!("{:?}", err.to_string()),
        };
    }));

    loop {

        sleep(Duration::new(0, 1000 * 1000 * 1000));

        // this method will call callbacks when their command executions are over.
        async_client.pump();
    }
    Ok(())
}

```

# License
Copyright (c) 2016 Gautier TANGUY

MIT License
