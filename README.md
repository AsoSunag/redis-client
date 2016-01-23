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

fn set_and_get()-> Result<String, RedisError> {
    let mut client = try!(redis_client::RedisClient::new("localhost", "6379"));
    let set_result: String = try!(client.set("key", "value"));
    let get_result: String = try!(client.get("key"));
    Ok(get_result)
}

```

# License
Copyright (c) 2016 Gautier TANGUY

MIT License
