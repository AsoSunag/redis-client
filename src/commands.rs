use errors::RedisError;
use redis::{RedisClient, RedisClientAsync};
use results::RedisResult;
use std::collections::HashMap;

/// A RedisCommand purpose is to build redis commands.
/// It can contains one or more commands for pipelining
///
/// Example:
///
/// ```
/// # use redis_client::commands::CommandBuilder;
/// let cmd = &mut redis_client::RedisCommand::new();
/// cmd.set("key", "value2").get("key");
/// ```
///
/// or its equivalent:
///
/// ```
/// let cmd = &mut redis_client::RedisCommand::new();
/// cmd.add_cmd("SET").add_arg("key").add_arg("value2").end().add_cmd("GET").add_arg("key").end();
/// ```
pub struct RedisCommand {
    cmd: Vec<u8>,
    cmd_nb: usize,
}

impl<'a> From<&'a mut RedisCommand> for &'a[u8] {
    fn from(command: &mut RedisCommand) -> &[u8] {
        &command.cmd[..]
    }
}

impl<'a> From<&'a mut RedisCommand> for Vec<u8> {
    fn from(command: &mut RedisCommand) -> Vec<u8> {
        command.cmd.clone()
    }
}

impl RedisCommand {
    pub fn new() -> RedisCommand {
        RedisCommand {
            cmd: vec![],
            cmd_nb: 0,
        }
    }

    /// Add a string representing the command (APPEND, GET, SET...) to the command. (Each command should start with this method)
    pub fn add_cmd<C>(&mut self, command: C) -> &mut RedisCommand where C: ToString {
        self.cmd.extend(command.to_string().into_bytes());
        self
    }

    /// Add a whitespace and a string to the commands
    pub fn add_arg<A>(&mut self, arg: A) -> &mut RedisCommand where A: ToString {
        self.cmd.extend([32].iter().cloned()); 
        self.cmd.extend(arg.to_string().into_bytes());
        self
    }

    /// Add a whitespace and a string for each one of the vector's items to the commands 
    pub fn add_args<A>(&mut self, args: Vec<A>) -> &mut RedisCommand where A: ToString {
        for arg in args {
            self.cmd.extend([32].iter().cloned()); 
            self.cmd.extend(arg.to_string().into_bytes());
        }
        self
    }

    /// Add a whitespace a key another whitespace and the value for each pair of the hash map to the curent command   
    pub fn add_arg_map<F: ToString>(&mut self, args: HashMap<String, F>) -> &mut RedisCommand {
        for (arg, value) in args {
            self.cmd.extend([32].iter().cloned()); 
            self.cmd.extend(arg.to_string().into_bytes());
            self.cmd.extend([32].iter().cloned()); 
            self.cmd.extend(value.to_string().into_bytes());
        }
        self
    }
    
    /// Add a whitespace and then an array of byte to the command        
    pub fn add_binary_arg(&mut self, arg: &[u8]) -> &mut RedisCommand {
        self.cmd.extend([32].iter().cloned()); 
        self.cmd.extend(arg.iter().cloned());
        self
    }

    /// Teminate a command
    pub fn end(&mut self) -> &mut RedisCommand {
        self.cmd.extend([13, 10].iter().cloned());
        self.cmd_nb += 1;
        self
    }

    /// Get the number of commands in the RedisCommand object
    pub fn get_command_nb(&self) -> usize {
        self.cmd_nb
    }
}

macro_rules! generate_command_traits {
    ($(
        fn $func_name:ident$(<$($gen_id:ident: $gen_type:ident),*>)*($($arg_name:ident: $arg_type:ty),*)  {
            $($cmd:ident $bo:expr;)+
        } 
    )*)
    => 
    (
        /// The trait CommandBuilder implements methods to abstract the construction of redis commands.
        ///
        /// So we can use:
        ///
        /// ```
        /// # use redis_client::commands::CommandBuilder;
        /// let cmd = &mut redis_client::RedisCommand::new();
        /// cmd.set("key", "value2");
        /// ```
        ///
        /// Instead of:
        ///
        /// ```
        /// let cmd = &mut redis_client::RedisCommand::new();
        /// cmd.add_cmd("SET").add_arg("key").add_arg("value2").end();
        /// ```
        pub trait CommandBuilder {
            $(
                fn $func_name$(<$($gen_id : $gen_type),*>)* (&mut self $(,$arg_name: $arg_type)*) -> &mut RedisCommand;
            )*
        }

        impl CommandBuilder for RedisCommand{
            $(
                fn $func_name$(<$($gen_id : $gen_type),*>)* (&mut self $(,$arg_name: $arg_type)*) -> &mut RedisCommand {
                    $(self.$cmd($bo));*;
                    self.end()
                }
            )*
        }

        /// The trait CommandSender implements methods to send redis commands and receive the response synchronously.
        ///
        /// Each methods returns a:
        ///
        /// ```plain
        /// Result<R: From<RedisResult>, RedisError>
        /// ```
        /// 
        /// It means that when calling a method from this trait you need to specify the type you want R to be.
        /// For example:
        ///
        /// ```no_run
        /// # use redis_client::commands::CommandSender;
        /// # fn function() -> Result<(), redis_client::errors::RedisError> {
        /// # let mut client = redis_client::redis::RedisClient::new("127.0.0.1", "6379").unwrap();
        /// let result: String = try!(client.set("key", "value"));
        /// # Ok(())}
        /// ```
        pub trait CommandSender {
            $(
                fn $func_name<R: From<RedisResult>, $($($gen_id : $gen_type),*)*> (&mut self $(,$arg_name: $arg_type)*) -> Result<R, RedisError>;
            )*
        }

        impl CommandSender for RedisClient{
            $(
                fn $func_name<R: From<RedisResult>, $($($gen_id : $gen_type),*)*> (&mut self $(,$arg_name: $arg_type)*) -> Result<R, RedisError> {
                    let cmd = &mut RedisCommand::new();
                    cmd.$func_name($($arg_name),*);

                    let res = try!(self.exec_redis_command(cmd));     
                    Ok(res.convert::<R>())
                }
            )*
        }

        /// The trait CommandSenderAsync implements methods to send redis commands and receive the response asynchronously.
        ///
        /// Each methods returns a:
        ///
        /// ```plain
        /// Result<(), RedisError>
        /// ```
        /// 
        /// It means that when calling a method from this trait if the Result is an error, the command execution failed to start.
        /// Otherwise it means that the command execution was successfully launched.
        /// 
        /// Each method will contained a callback argument:
        ///
        /// ```plain
        /// Fn(Result<RedisResult, RedisError>)
        /// ```
        /// Once the command execution is over, it will be called once the pump method is called.
        /// 
        /// All commands execution are made in a background thread. 
        pub trait CommandSenderAsync {
            $(
                fn $func_name<G: Fn(Result<RedisResult, RedisError>), $($($gen_id : $gen_type),*)*> (&mut self $(,$arg_name: $arg_type)*, callback: G) 
                    -> Result<(), RedisError> where G: Send + 'static;
            )*
        }

        impl CommandSenderAsync for RedisClientAsync{
            $(
                fn $func_name<G: Fn(Result<RedisResult, RedisError>), $($($gen_id : $gen_type),*)*> (&mut self $(,$arg_name: $arg_type)*, callback: G) 
                    -> Result<(), RedisError> where G: Send + 'static 
                {
                    let cmd = &mut RedisCommand::new();
                    cmd.$func_name($($arg_name),*);

                    try!(self.exec_redis_command_async(cmd, callback));     
                    Ok(())
                }
            )*
        }
    )
}

generate_command_traits!{
    fn append<K: ToString, V: ToString>(key: K, value: V) {
        add_cmd("APPEND");
        add_arg(key);
        add_arg(value);
    }

    fn auth<P: ToString>(password: P) {
        add_cmd("AUTH");
        add_arg(password);
    }

    fn bgrewriteaof() {
        add_cmd("BGREWRITEAOF");
    }

    fn bgsave() {
        add_cmd("BGSAVE");
    }

    fn bitcount<K: ToString>(key: K) {
        add_cmd("BITCOUNT");
        add_arg(key);
    }

    fn bitcount_range<K: ToString>(key: K, start_range: i64, end_range: i64) {
        add_cmd("BITCOUNT");
        add_arg(key);
        add_arg(start_range);
        add_arg(end_range);
    }

    fn blpop<K: ToString>(key: K, timeout: u32) {
        add_cmd("BLPOP");
        add_arg(key);
        add_arg(timeout);
    }

    fn mblpop<K: ToString>(keys: Vec<K>, timeout: u32) {
        add_cmd("BLPOP");
        add_args(keys);
        add_arg(timeout);
    }

    fn brpop<K: ToString>(key: K, timeout: u32) {
        add_cmd("BRPOP");
        add_arg(key);
        add_arg(timeout);
    }

    fn mbrpop<K: ToString>(keys: Vec<K>, timeout: u32) {
        add_cmd("BRPOP");
        add_args(keys);
        add_arg(timeout);
    }

    fn brpoplpush<S: ToString, D: ToString>(source: S, dest: D, timeout: u32) {
        add_cmd("BRPOPLPUSH");
        add_arg(source);
        add_arg(dest);
        add_arg(timeout);
    }

    fn decr<K: ToString>(key: K) {
        add_cmd("DECR");
        add_arg(key);
    }

    fn decrby<K: ToString>(key: K, increment: i64) {
        add_cmd("DECRBY");
        add_arg(key);
        add_arg(increment);
    }

    fn del<K: ToString>(key: K) {
        add_cmd("DEL");
        add_arg(key);
    }

    fn mdel<K: ToString>(keys: Vec<K>){
        add_cmd("DEL");
        add_args(keys);
    }

    fn discard() {
        add_cmd("DISCARD");
    }

    fn echo<K: ToString>(msg: K) {
        add_cmd("ECHO");
        add_arg(msg);
    }

    fn exec() {
        add_cmd("EXEC");
    }

    fn exists<K: ToString>(key: K) {
        add_cmd("EXISTS");
        add_arg(key);
    }

    fn mexists<K: ToString>(keys: Vec<K>){
        add_cmd("EXISTS");
        add_args(keys);
    }

    fn expire<K: ToString>(key: K, expiry: i64) {
        add_cmd("EXPIRE");
        add_arg(key);
        add_arg(expiry);
    }

    fn expireat<K: ToString>(key: K, timestamp: i64) {
        add_cmd("EXPIREAT");
        add_arg(key);
        add_arg(timestamp);
    }

    fn get<K: ToString>(key: K) {
        add_cmd("GET");
        add_arg(key);
    }

    fn getrange<K: ToString>(key: K, start_range: i64, end_range: i64) {
        add_cmd("GETRANGE");
        add_arg(key);
        add_arg(start_range);
        add_arg(end_range);
    }

    fn hdel<K: ToString, F: ToString>(key: K, field: F) {
        add_cmd("HDEL");
        add_arg(key);
        add_arg(field);
    }

    fn hmdel<K: ToString, V: ToString>(key: K, fields: Vec<V>) {
        add_cmd("HDEL");
        add_arg(key);
        add_args(fields);
    }

    fn hexists<K: ToString, F: ToString>(key: K, field: F) {
        add_cmd("HEXISTS");
        add_arg(key);
        add_arg(field);
    }

    fn hget<K: ToString, F: ToString>(key: K, field: F) {
        add_cmd("HGET");
        add_arg(key);
        add_arg(field);
    }

    fn hgetall<K: ToString>(key: K) {
        add_cmd("HGETALL");
        add_arg(key);
    }

    fn hincrby<K: ToString, F: ToString>(key: K, field: F, increment: i64) {
        add_cmd("HINCRBY");
        add_arg(key);
        add_arg(field);
        add_arg(increment);
    }

    fn hincrbyfloat<K: ToString, F: ToString>(key: K, field: F, increment: f64) {
        add_cmd("HINCRBYBYFLOAT");
        add_arg(key);
        add_arg(field);
        add_arg(increment);
    }

    fn hkeys<K: ToString>(key: K) {
        add_cmd("HKEYS");
        add_arg(key);
    }

    fn hlen<K: ToString>(key: K) {
        add_cmd("HLEN");
        add_arg(key);
    }

    fn hmget<K: ToString, F: ToString>(key: K, fields: Vec<F>) {
        add_cmd("HMGET");
        add_arg(key);
        add_args(fields);
    }

    fn hmset<K: ToString>(key: K, fields: HashMap<String, K>) {
        add_cmd("HMSET");
        add_arg(key);
        add_arg_map(fields);
    }

    fn hset<K: ToString, F: ToString, V: ToString>(key: K, field: F, value: V) {
        add_cmd("HSET");
        add_arg(key);
        add_arg(field);
        add_arg(value);
    }

    fn hstrlen<K: ToString, F: ToString>(key: K, field: F) {
        add_cmd("HSTRLEN");
        add_arg(key);
        add_arg(field);
    }

    fn hsetnx<K: ToString, F: ToString, V: ToString>(key: K, field: F, value: V) {
        add_cmd("HSETNX");
        add_arg(key);
        add_arg(field);
        add_arg(value);
    }

    fn hvals<K: ToString>(key: K) {
        add_cmd("HVALS");
        add_arg(key);
    }

    fn lindex<K: ToString>(key: K, index: i32) {
        add_cmd("LINDEX");
        add_arg(key);
        add_arg(index);
    }

    fn linsert_after<K: ToString, P: ToString, V: ToString>(key: K, pivot: P, value: V) {
        add_cmd("LINSERT");
        add_arg(key);
        add_arg("AFTER");
        add_arg(pivot);
        add_arg(value);
    }

    fn linsert_before<K: ToString, P: ToString, V: ToString>(key: K, pivot: P, value: V) {
        add_cmd("LINSERT");
        add_arg(key);
        add_arg("BEFORE");
        add_arg(pivot);
        add_arg(value);
    }

    fn llen<K: ToString>(key: K) {
        add_cmd("LLEN");
        add_arg(key);
    }

    fn lpop<K: ToString>(key: K) {
        add_cmd("LPOP");
        add_arg(key);
    }

    fn lpush<K: ToString, V: ToString>(key: K, value: V) {
        add_cmd("LPUSH");
        add_arg(key);
        add_arg(value);
    }

    fn mlpush<K: ToString, V: ToString>(key: K, values: Vec<V>) {
        add_cmd("LPUSH");
        add_arg(key);
        add_args(values);
    }

    fn lpushx<K: ToString, V: ToString>(key: K, value: V) {
        add_cmd("LPUSHX");
        add_arg(key);
        add_arg(value);
    }

    fn lrange<K: ToString>(key: K, start: i32, end: i32) {
        add_cmd("LRANGE");
        add_arg(key);
        add_arg(start);
        add_arg(end);
    }

    fn lrem<K: ToString, V: ToString>(key: K, count: i32, value: V) {
        add_cmd("LREM");
        add_arg(key);
        add_arg(count);
        add_arg(value);
    }

    fn lset<K: ToString, V: ToString>(key: K, index: i32, value: V) {
        add_cmd("LSET");
        add_arg(key);
        add_arg(index);
        add_arg(value);
    }

    fn ltrim<K: ToString>(key: K, start: i32, end: i32) {
        add_cmd("LTRIM");
        add_arg(key);
        add_arg(start);
        add_arg(end);
    }

    fn multi() {
        add_cmd("MULTI");
    }

    fn rename<K: ToString, N: ToString>(key: K, new_key: N) {
        add_cmd("RENAME");
        add_arg(key);
        add_arg(new_key);
    }

    fn renamenx<K: ToString, N: ToString>(key: K, new_key: N) {
        add_cmd("RENAMENX");
        add_arg(key);
        add_arg(new_key);
    }

    fn rpop<K: ToString>(key: K) {
        add_cmd("RPOP");
        add_arg(key);
    }

    fn rpoplpush<S: ToString, D: ToString>(source: S, dest: D) {
        add_cmd("RPOPLPUSH");
        add_arg(source);
        add_arg(dest);
    }

    fn rpush<K: ToString, V: ToString>(key: K, value: V) {
        add_cmd("RPUSH");
        add_arg(key);
        add_arg(value);
    }

    fn mrpush<K: ToString, V: ToString>(key: K, values: Vec<V>) {
        add_cmd("RPUSH");
        add_arg(key);
        add_args(values);
    }

    fn rpushx<K: ToString, V: ToString>(key: K, value: V) {
        add_cmd("RPUSHX");
        add_arg(key);
        add_arg(value);
    }

    fn sadd<K: ToString, M: ToString>(key: K, member: M) {
        add_cmd("SADD");
        add_arg(key);
        add_arg(member);
    }

    fn msadd<K: ToString, M: ToString>(key: K, members: Vec<M>) {
        add_cmd("SADD");
        add_arg(key);
        add_args(members);
    }

    fn sadd_binary<K: ToString>(key: K, member: &[u8]) {
        add_cmd("SADD");
        add_arg(key);
        add_binary_arg(member);
    }

    fn scard<K: ToString>(key: K) {
        add_cmd("SCARD");
        add_arg(key);
    }

    fn select(db_index: i32){
        add_cmd("SELECT");
        add_arg(db_index);
    }

    fn set<K: ToString, V: ToString>(key: K, value: V) {
        add_cmd("SET");
        add_arg(key);
        add_arg(value);
    }

    fn set_binary<K: ToString>(key: K, value: &[u8]) {
        add_cmd("SET");
        add_arg(key);
        add_binary_arg(value);
    }

    fn setex<K: ToString, V: ToString>(key: K, value: V, expiry: i64) {
        add_cmd("SET");
        add_arg(key);
        add_arg(value);
        add_arg("EX");
        add_arg(expiry);
    }

    fn psetex<K: ToString, V: ToString>(key: K, value: V, expiry: i64) {
        add_cmd("SET");
        add_arg(key);
        add_arg(value);
        add_arg("PX");
        add_arg(expiry);
    }

    fn setnx<K: ToString, V: ToString>(key: K, value: V) {
        add_cmd("SET");
        add_arg(key);
        add_arg(value);
        add_arg("NX");
    }

    fn setxx<K: ToString, V: ToString>(key: K, value: V) {
        add_cmd("SET");
        add_arg(key);
        add_arg(value);
        add_arg("XX");
    }

    fn setex_nx<K: ToString, V: ToString>(key: K, value: V, expiry: i64) {
        add_cmd("SET");
        add_arg(key);
        add_arg(value);
        add_arg("EX");
        add_arg(expiry);
        add_arg("NX");
    }

    fn setex_xx<K: ToString, V: ToString>(key: K, value: V, expiry: i64) {
        add_cmd("SET");
        add_arg(key);
        add_arg(value);
        add_arg("EX");
        add_arg(expiry);
        add_arg("XX");
    }

    fn psetex_nx<K: ToString, V: ToString>(key: K, value: V, expiry: i64) {
        add_cmd("SET");
        add_arg(key);
        add_arg(value);
        add_arg("PX");
        add_arg(expiry);
        add_arg("NX");
    }

    fn psetex_xx<K: ToString, V: ToString>(key: K, value: V, expiry: i64) {
        add_cmd("SET");
        add_arg(key);
        add_arg(value);
        add_arg("PX");
        add_arg(expiry);
        add_arg("XX");
    }

    fn setbit<K: ToString>(key: K, offset: u32, bit: u8) {
        add_cmd("SETBIT");
        add_arg(key);
        add_arg(offset);
        add_arg(bit);
    }

    fn setrange<K: ToString, V: ToString>(key: K, offset: u32, value: V) {
        add_cmd("SETRANGE");
        add_arg(key);
        add_arg(offset);
        add_arg(value);
    }

    fn sismember<K: ToString, M: ToString>(key: K, member: M) {
        add_cmd("SISMEMBER");
        add_arg(key);
        add_arg(member);
    }

    fn smembers<K: ToString>(key: K) {
        add_cmd("SMEMBERS");
        add_arg(key);
    }

    fn spop<K: ToString>(key: K) {
        add_cmd("SPOP");
        add_arg(key);
    }

    fn spop_count<K: ToString>(key: K, count: u32) {
        add_cmd("SPOP");
        add_arg(key);
        add_arg(count);
    }

    fn srem<K: ToString, M: ToString>(key: K, member: M) {
        add_cmd("SREM");
        add_arg(key);
        add_arg(member);
    }

    fn msrem<K: ToString, M: ToString>(key: K, members: Vec<M>) {
        add_cmd("SREM");
        add_arg(key);
        add_args(members);
    }

    fn strlen<K: ToString>(key: K) {
        add_cmd("STRLEN");
        add_arg(key);
    }

    fn ttl<K: ToString>(key: K) {
        add_cmd("TTL");
        add_arg(key);
    }

    fn unwatch() {
        add_cmd("UNWATCH");
    }

    fn watch<K: ToString>(key: K) {
        add_cmd("WATCH");
        add_arg(key);
    }

    fn mwatch<K: ToString>(keys: Vec<K>) {
        add_cmd("WATCH");
        add_args(keys);
    }

    fn zadd<K: ToString, V: ToString>(key: K, score: f64, member: V) {
        add_cmd("ZADD");
        add_arg(key);
        add_arg(score);
        add_arg(member);
    }

    fn zadd_binary<K: ToString>(key: K, score: f64, member: &[u8]) {
        add_cmd("ZADD");
        add_arg(key);
        add_arg(score);
        add_binary_arg(member);
    }

    fn zaddnx<K: ToString, V: ToString>(key: K, score: f64, member: V) {
        add_cmd("ZADD");
        add_arg(key);
        add_arg("NX");
        add_arg(score);
        add_arg(member);
    }

    fn zaddxx<K: ToString, V: ToString>(key: K, score: f64, member: V) {
        add_cmd("ZADD");
        add_arg(key);
        add_arg("XX");
        add_arg(score);
        add_arg(member);
    }

    fn zaddnx_ch<K: ToString, V: ToString>(key: K, score: f64, member: V) {
        add_cmd("ZADD");
        add_arg(key);
        add_arg("NX");
        add_arg("CH");
        add_arg(score);
        add_arg(member);
    }

    fn zaddxx_ch<K: ToString, V: ToString>(key: K, score: f64, member: V) {
        add_cmd("ZADD");
        add_arg(key);
        add_arg("XX");
        add_arg("CH");
        add_arg(score);
        add_arg(member);
    }

    fn zcard<K: ToString>(key: K) {
        add_cmd("ZCARD");
        add_arg(key);
    }

    fn zcount<K: ToString, S: ToString, E: ToString>(key: K, start_range: S, end_range: E) {
        add_cmd("ZCOUNT");
        add_arg(key);
        add_arg(start_range);
        add_arg(end_range);
    }

    fn zincrby<K: ToString, V: ToString>(key: K, increment: f64, member: V) {
        add_cmd("ZINCRBY");
        add_arg(key);
        add_arg(increment);
        add_arg(member);
    }

    fn zlexcount<K: ToString, S: ToString, E: ToString>(key: K, min: S, max: E) {
        add_cmd("ZLEXCOUNT");
        add_arg(key);
        add_arg(min);
        add_arg(max);
    }

    fn zrem<K: ToString, M: ToString>(key: K, member: M) {
        add_cmd("ZREM");
        add_arg(key);
        add_arg(member);
    }

    fn mzrem<K: ToString, M: ToString>(key: K, members: Vec<M>) {
        add_cmd("ZREM");
        add_arg(key);
        add_args(members);
    }

    fn zrange<K: ToString>(key: K, start_range: i64, end_range: i64) {
        add_cmd("ZRANGE");
        add_arg(key);
        add_arg(start_range);
        add_arg(end_range);
    }

    fn zrange_with_scores<K: ToString>(key: K, start_range: i64, end_range: i64) {
        add_cmd("ZRANGE");
        add_arg(key);
        add_arg(start_range);
        add_arg(end_range);
        add_arg("WITHSCORES");
    }

    fn zrevrange<K: ToString>(key: K, start_range: i64, end_range: i64) {
        add_cmd("ZREVRANGE");
        add_arg(key);
        add_arg(start_range);
        add_arg(end_range);
    }

    fn zrevrange_with_scores<K: ToString>(key: K, start_range: i64, end_range: i64) {
        add_cmd("ZREVRANGE");
        add_arg(key);
        add_arg(start_range);
        add_arg(end_range);
        add_arg("WITHSCORES");
    }
        
}

