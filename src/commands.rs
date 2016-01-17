use std::collections::HashMap;

pub struct RedisCommand {
    cmd: Vec<u8>,
}

impl<'a> From<&'a mut RedisCommand> for &'a[u8] {
    fn from(command: &mut RedisCommand) -> &[u8] {
        &command.cmd[..]
    }
}

impl RedisCommand {
    pub fn new() -> RedisCommand {
        RedisCommand {
            cmd: vec![],
        }
    }

    pub fn add_cmd<C>(&mut self, command: C) -> &mut RedisCommand where C: ToString {
        self.cmd.extend(command.to_string().into_bytes());
        self
    }

    pub fn add_arg<A>(&mut self, arg: A) -> &mut RedisCommand where A: ToString{
        self.cmd.extend([32].iter().cloned()); 
        self.cmd.extend(arg.to_string().into_bytes());
        self
    }

    pub fn add_binary_arg(&mut self, arg: &[u8]) -> &mut RedisCommand {
        self.cmd.extend([32].iter().cloned()); 
        self.cmd.extend(arg.iter().cloned());
        self
    }

    pub fn end(&mut self) -> &mut RedisCommand {
        self.cmd.extend([13, 10].iter().cloned());
        self
    }

    pub fn append<K, V>(&mut self, key: K, value: V) -> &mut RedisCommand where K: ToString, V: ToString {
        self.add_cmd("APPEND").add_arg(key).add_arg(value).end()
    }

    pub fn auth<P>(&mut self, password: P) -> &mut RedisCommand where P: ToString {
        self.add_cmd("AUTH").add_arg(password).end()
    }

    pub fn bgrewriteaof(&mut self) -> &mut RedisCommand {
        self.add_cmd("BGREWRITEAOF").end()
    }

    pub fn bgsave(&mut self) -> &mut RedisCommand {
        self.add_cmd("BGSAVE").end()
    }

    pub fn bitcount<K>(&mut self, key: K) -> &mut RedisCommand where K: ToString {
        self.add_cmd("BITCOUNT").add_arg(key).end()
    }

    pub fn bitcount_range<K>(&mut self, key: K, start_range: i64, end_range: i64) -> &mut RedisCommand where K: ToString {
        self.add_cmd("BITCOUNT")
        .add_arg(key)
        .add_arg(start_range)
        .add_arg(end_range)
        .end()
    }

    pub fn del<K>(&mut self, key: K) -> &mut RedisCommand where K: ToString {
        self.add_cmd("DEL").add_arg(key).end()
    }

    pub fn mdel(&mut self, keys: Vec<String>) -> &mut RedisCommand {
        self.add_cmd("DEL");
        for key in keys {
            self.add_arg(key);
        }
        self.end()
    }

    pub fn exists<K>(&mut self, key: K) -> &mut RedisCommand where K: ToString {
        self.add_cmd("EXISTS").add_arg(key).end()
    }

    pub fn mexists(&mut self, keys: Vec<String>) -> &mut RedisCommand {
        self.add_cmd("EXISTS");
        for key in keys {
            self.add_arg(key);
        }
        self.end()
    }

    pub fn expire<K>(&mut self, key: K, expiry: i64) -> &mut RedisCommand where K: ToString {
        self.add_cmd("EXPIRE").add_arg(key).add_arg(expiry).end()
    }

    pub fn expireat<K>(&mut self, key: K, timestamp: i64) -> &mut RedisCommand where K: ToString {
        self.add_cmd("EXPIREAT").add_arg(key).add_arg(timestamp).end()
    }

    pub fn get<K>(&mut self, key: K) -> &mut RedisCommand where K: ToString {
        self.add_cmd("GET").add_arg(key).end()
    }

    pub fn getrange<K>(&mut self, key: K, start_range: i64, end_range: i64) -> &mut RedisCommand where K: ToString {
        self.add_cmd("GETRANGE")
        .add_arg(key)
        .add_arg(start_range)
        .add_arg(end_range)
        .end()
    }

    pub fn hdel<K, F>(&mut self, key: K, field: F) -> &mut RedisCommand where K: ToString, F: ToString {
        self.add_cmd("HDEL").add_arg(key).add_arg(field).end()
    }

    pub fn hmdel<K>(&mut self, key: K, fields: Vec<String>) -> &mut RedisCommand where K: ToString {
        self.add_cmd("HDEL").add_arg(key);
        for field in fields {
            self.add_arg(field);
        }
        self.end()
    }

    pub fn hexists<K, F>(&mut self, key: K, field: F) -> &mut RedisCommand where K: ToString, F: ToString {
        self.add_cmd("HEXISTS").add_arg(key).add_arg(field).end()
    }

    pub fn hget<K, F>(&mut self, key: K, field: F) -> &mut RedisCommand where K: ToString, F: ToString {
        self.add_cmd("HGET").add_arg(key).add_arg(field).end()
    }

    pub fn hgetall<K>(&mut self, key: K) -> &mut RedisCommand where K: ToString {
        self.add_cmd("HGETALL").add_arg(key).end()
    }

    pub fn hincrby<K, F>(&mut self, key: K, field: F, increment: i64) -> &mut RedisCommand where K: ToString, F: ToString {
        self.add_cmd("HINCRBY").add_arg(key).add_arg(field).add_arg(increment).end()
    }

    pub fn hincrbyfloat<K, F>(&mut self, key: K, field: F, increment: f64) -> &mut RedisCommand where K: ToString, F: ToString {
        self.add_cmd("HINCRBYBYFLOAT").add_arg(key).add_arg(field).add_arg(increment).end()
    }

    pub fn hkeys<K>(&mut self, key: K) -> &mut RedisCommand where K: ToString {
        self.add_cmd("HKEYS").add_arg(key).end()
    }

    pub fn hlen<K>(&mut self, key: K) -> &mut RedisCommand where K: ToString {
        self.add_cmd("HLEN").add_arg(key).end()
    }

    pub fn hmget<K>(&mut self, key: K, fields: Vec<String>) -> &mut RedisCommand where K: ToString {
        self.add_cmd("HMGET").add_arg(key);
        for field in fields {
            self.add_arg(field);
        }
        self.end()
    }

    pub fn hmset<K>(&mut self, key: K, fields: HashMap<String, String>) -> &mut RedisCommand where K: ToString {
        self.add_cmd("HMSET").add_arg(key);
        for (field, value) in fields {
            self.add_arg(field).add_arg(value);
        }
        self.end()    
    }

    pub fn hset<K, F, V>(&mut self, key: K, field: F, value: V) -> &mut RedisCommand where K: ToString, F: ToString, V: ToString {
        self.add_cmd("HSET").add_arg(key).add_arg(field).add_arg(value).end()
    }

    pub fn hstrlen<K, F, V>(&mut self, key: K, field: F) -> &mut RedisCommand where K: ToString, F: ToString, V: ToString {
        self.add_cmd("HSTRLEN").add_arg(key).add_arg(field).end()
    }

    pub fn hsetnx<K, F, V>(&mut self, key: K, field: F, value: V) -> &mut RedisCommand where K: ToString, F: ToString, V: ToString {
        self.add_cmd("HSETNX").add_arg(key).add_arg(field).add_arg(value).end()
    }

    pub fn hvals<K>(&mut self, key: K) -> &mut RedisCommand where K: ToString {
        self.add_cmd("HVALS").add_arg(key).end()
    }

    pub fn select(&mut self, db_index: u32) -> &mut RedisCommand {
        self.add_cmd("SELECT").add_arg(db_index).end()
    }

    pub fn set<K, V>(&mut self, key: K, value: V) -> &mut RedisCommand where K: ToString, V: ToString {
        self.add_cmd("SET").add_arg(key).add_arg(value).end()
    }

    pub fn set_binary<K>(&mut self, key: K, value: &[u8]) -> &mut RedisCommand where K: ToString {
        self.add_cmd("SET").add_arg(key).add_binary_arg(value)
    }

    pub fn setex<K, V>(&mut self, key: K, value: String, expiry: i64) -> &mut RedisCommand where K: ToString, V: ToString  {
        self.add_cmd("SET").add_arg(key).add_arg(value).add_arg("EX").add_arg(expiry).end()
    }

    pub fn psetex<K, V>(&mut self, key: K, value: String, expiry: i64) -> &mut RedisCommand where K: ToString, V: ToString  {
        self.add_cmd("SET").add_arg(key).add_arg(value).add_arg("PX").add_arg(expiry).end()
    }

    pub fn setnx<K, V>(&mut self, key: K, value: String) -> &mut RedisCommand where K: ToString, V: ToString {
        self.add_cmd("SET").add_arg(key).add_arg(value).add_arg("NX").end()
    }

    pub fn setxx<K, V>(&mut self, key: K, value: String) -> &mut RedisCommand where K: ToString, V: ToString {
        self.add_cmd("SET").add_arg(key).add_arg(value).add_arg("XX").end()
    }

    pub fn setex_nx<K, V>(&mut self, key: K, value: String, expiry: i64) -> &mut RedisCommand where K: ToString, V: ToString {
        self.add_cmd("SET").add_arg(key).add_arg(value).add_arg("EX").add_arg(expiry).add_arg("NX").end()
    }

    pub fn setex_xx<K, V>(&mut self, key: K, value: String, expiry: i64) -> &mut RedisCommand where K: ToString, V: ToString {
        self.add_cmd("SET").add_arg(key).add_arg(value).add_arg("EX").add_arg(expiry).add_arg("NX").end()
    }

    pub fn psetex_nx<K, V>(&mut self, key: K, value: String, expiry: i64) -> &mut RedisCommand where K: ToString, V: ToString {
        self.add_cmd("SET").add_arg(key).add_arg(value).add_arg("PX").add_arg(expiry).add_arg("NX").end()
    }

    pub fn psetex_xx<K, V>(&mut self, key: K, value: String, expiry: i64) -> &mut RedisCommand where K: ToString, V: ToString {
        self.add_cmd("SET").add_arg(key).add_arg(value).add_arg("PX").add_arg(expiry).add_arg("XX").end()
    }

    pub fn ttl<K>(&mut self, key: K) -> &mut RedisCommand where K: ToString {
        self.add_cmd("TTL").add_arg(key).end()
    }

    pub fn zadd<K, V>(&mut self, key: K, score: f64, member: V) -> &mut RedisCommand where K: ToString, V: ToString {
        self.add_cmd("ZADD").add_arg(key).add_arg(score).add_arg(member).end()
    }

    pub fn zadd_binary<K, A>(&mut self, key: K, score: f64, member: &[u8], args: A) -> &mut RedisCommand where K: ToString, A: ToString {
        self.add_cmd("ZADD").add_arg(key).add_arg(score).add_binary_arg(member).end()
    }

    pub fn zaddnx<K, V>(&mut self, score: f64, key: K, member: V) -> &mut RedisCommand where K: ToString, V: ToString {
        self.add_cmd("ZADD").add_arg(key).add_arg("NX").add_arg(score).add_arg(member).end()
    }

    pub fn zaddxx<K, V>(&mut self, score: f64, key: K, member: V) -> &mut RedisCommand where K: ToString, V: ToString {
        self.add_cmd("ZADD").add_arg(key).add_arg("XX").add_arg(score).add_arg(member).end()
    }

    pub fn zaddnx_ch<K, V>(&mut self, score: f64, key: K, member: V) -> &mut RedisCommand where K: ToString, V: ToString {
        self.add_cmd("ZADD").add_arg(key).add_arg("NX").add_arg("CH").add_arg(score).add_arg(member).end()
    }

    pub fn zaddxx_ch<K, V>(&mut self, score: f64, key: K, member: V) -> &mut RedisCommand where K: ToString, V: ToString {
        self.add_cmd("ZADD").add_arg(key).add_arg("XX").add_arg("CH").add_arg(score).add_arg(member).end()
    }

    pub fn zcard<K>(&mut self, key: K) -> &mut RedisCommand where K: ToString {
        self.add_cmd("ZCARD").add_arg(key).end()
    }

    pub fn zcount<K, S, E>(&mut self, key: K, start_range: S, end_range: E) -> &mut RedisCommand where K: ToString, S: ToString, E: ToString  {
        self.add_cmd("ZCOUNT").add_arg(key).add_arg(start_range).add_arg(end_range).end()
    }

    pub fn zincrby<K, V>(&mut self, key: K, increment: f64, member: V) -> &mut RedisCommand where K: ToString, V: ToString {
        self.add_cmd("ZINCRBY").add_arg(key).add_arg(increment).add_arg(member).end()
    }

    pub fn zrange<K, S, E>(&mut self, key: K, start_range: S, end_range: E) -> &mut RedisCommand where K: ToString, S: ToString, E: ToString {
        self.add_cmd("ZRANGE").add_arg(key).add_arg(start_range).add_arg(end_range).end()
    }

    pub fn zrange_with_scores<K, S, E>(&mut self, key: K, start_range: S, end_range: E) -> &mut RedisCommand where K: ToString, S: ToString, E: ToString {
        self.add_cmd("ZRANGE").add_arg(key).add_arg(start_range).add_arg(end_range).add_arg("WITHSCORES").end()
    }
}

