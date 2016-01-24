//! The tests `commands` mod is checking if the output of each command is correct. 

extern crate redis_client;

use redis_client::commands::CommandBuilder;
use redis_client::commands::RedisCommand;

use std::collections::HashMap;


/// This function checks two array of bytes.
///
/// # Examples
///
/// ``` rust
/// let cmd = &mut RedisCommand::new();
/// cmd.append("key", "value");
///
/// check_result(cmd.into(), b"APPEND key value\r\n");
/// ```
fn check_result(result: &[u8], expected: &[u8]) {
    assert_eq!( result, expected);
}

#[test]
fn append_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.append("key", "value");

    check_result(cmd.into(), b"APPEND key value\r\n");
}

#[test]
fn auth_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.auth("password");

    check_result(cmd.into(), b"AUTH password\r\n");
}

#[test]
fn bgrewriteaof_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.bgrewriteaof();

    check_result(cmd.into(), b"BGREWRITEAOF\r\n");
}

#[test]
fn bgsave_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.bgsave();

    check_result(cmd.into(), b"BGSAVE\r\n");
}

#[test]
fn bitcount_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.bitcount("key");

    check_result(cmd.into(), b"BITCOUNT key\r\n");
}

#[test]
fn bitcount_range_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.bitcount_range("key", -1, 1);

    check_result(cmd.into(), b"BITCOUNT key -1 1\r\n");
}

#[test]
fn del_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.del("key");

    check_result(cmd.into(), b"DEL key\r\n");
}

#[test]
fn mdel_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.mdel(vec!["key1", "key2"]);

    check_result(cmd.into(), b"DEL key1 key2\r\n");
}

#[test]
fn discard_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.discard();

    check_result(cmd.into(), b"DISCARD\r\n");
}

#[test]
fn exec_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.exec();

    check_result(cmd.into(), b"EXEC\r\n");
}

#[test]
fn exists_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.exists("key");

    check_result(cmd.into(), b"EXISTS key\r\n");
}

#[test]
fn mexists_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.mexists(vec!["key1", "key2"]);

    check_result(cmd.into(), b"EXISTS key1 key2\r\n");
}

#[test]
fn expire_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.expire("key", 42);

    check_result(cmd.into(), b"EXPIRE key 42\r\n");
}

#[test]
fn expireat_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.expireat("key", 42);

    check_result(cmd.into(), b"EXPIREAT key 42\r\n");
}

#[test]
fn get_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.get("key");

    check_result(cmd.into(), b"GET key\r\n");
}

#[test]
fn getrange_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.getrange("key", -1, 1);

    check_result(cmd.into(), b"GETRANGE key -1 1\r\n");
}

#[test]
fn hdel_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.hdel("key", "field");

    check_result(cmd.into(), b"HDEL key field\r\n");
}

#[test]
fn mhdel_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.hmdel("key", vec!["field1", "field2"]);

    check_result(cmd.into(), b"HDEL key field1 field2\r\n");
}

#[test]
fn hexists_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.hexists("key", "field");

    check_result(cmd.into(), b"HEXISTS key field\r\n");
}

#[test]
fn hget_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.hget("key", "field");

    check_result(cmd.into(), b"HGET key field\r\n");
}

#[test]
fn hgetall_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.hgetall("key");

    check_result(cmd.into(), b"HGETALL key\r\n");
}

#[test]
fn hincrby_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.hincrby("key", "field", 1);

    check_result(cmd.into(), b"HINCRBY key field 1\r\n");
}

#[test]
fn hincrbyfloat_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.hincrbyfloat("key", "value", 4.2);

    check_result(cmd.into(), b"HINCRBYBYFLOAT key value 4.2\r\n");
}

#[test]
fn hkeys_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.hkeys("key");

    check_result(cmd.into(), b"HKEYS key\r\n");
}

#[test]
fn hlen_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.hlen("key");

    check_result(cmd.into(), b"HLEN key\r\n");
}

#[test]
fn hmget_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.hmget("key", vec!["field1", "field2"]);

    check_result(cmd.into(), b"HMGET key field1 field2\r\n");
}

#[test]
fn hmset_cmd_works() {
    let cmd = &mut RedisCommand::new();
    let mut fields = HashMap::new();
    fields.insert("field1".to_string(), "value1");
    // FIXME order of the key value pair is not fixed
    //fields.insert("field2".to_string(), "value2");
    cmd.hmset("key", fields);

    check_result(cmd.into(), b"HMSET key field1 value1\r\n");
}

#[test]
fn hset_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.hset("key", "field", "value");

    check_result(cmd.into(), b"HSET key field value\r\n");
}

#[test]
fn hstrlen_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.hstrlen("key", "field");

    check_result(cmd.into(), b"HSTRLEN key field\r\n");
}

#[test]
fn hsetnx_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.hsetnx("key", "field", "value");

    check_result(cmd.into(), b"HSETNX key field value\r\n");
}

#[test]
fn hvals_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.hvals("key");

    check_result(cmd.into(), b"HVALS key\r\n");
}

#[test]
fn multi_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.multi();

    check_result(cmd.into(), b"MULTI\r\n");
}

#[test]
fn sadd_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.sadd("key", "member");

    check_result(cmd.into(), b"SADD key member\r\n");
}

#[test]
fn msadd_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.msadd("key", vec!["member1", "member2"]);

    check_result(cmd.into(), b"SADD key member1 member2\r\n");
}

#[test]
fn sadd_binary_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.sadd_binary("key", b"member");

    check_result(cmd.into(), b"SADD key member\r\n");
}

#[test]
fn scard_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.scard("key");

    check_result(cmd.into(), b"SCARD key\r\n");
}

#[test]
fn select_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.select(1);

    check_result(cmd.into(), b"SELECT 1\r\n");
}

#[test]
fn set_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.set("key", "value");

    check_result(cmd.into(), b"SET key value\r\n");
}

#[test]
fn set_binary_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.set_binary("key", b"value");

    check_result(cmd.into(), b"SET key value\r\n");
}

#[test]
fn setex_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.setex("key", "value", 42);

    check_result(cmd.into(), b"SET key value EX 42\r\n");
}

#[test]
fn psetex_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.psetex("key", "value", 42);

    check_result(cmd.into(), b"SET key value PX 42\r\n");
}

#[test]
fn setnx_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.setnx("key", "value");

    check_result(cmd.into(), b"SET key value NX\r\n");
}

#[test]
fn setxx_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.setxx("key", "value");

    check_result(cmd.into(), b"SET key value XX\r\n");
}

#[test]
fn setex_nx_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.setex_nx("key", "value", 42);

    check_result(cmd.into(), b"SET key value EX 42 NX\r\n");
}

#[test]
fn setex_xx_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.setex_xx("key", "value", 42);

    check_result(cmd.into(), b"SET key value EX 42 XX\r\n");
}

#[test]
fn psetex_nx_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.psetex_nx("key", "value", 42);

    check_result(cmd.into(), b"SET key value PX 42 NX\r\n");
}

#[test]
fn psetex_xx_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.psetex_xx("key", "value", 42);

    check_result(cmd.into(), b"SET key value PX 42 XX\r\n");
}

#[test]
fn setbit_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.setbit("key", 42, 0);

    check_result(cmd.into(), b"SETBIT key 42 0\r\n");
}

#[test]
fn setrange_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.setrange("key", 42, "value");

    check_result(cmd.into(), b"SETRANGE key 42 value\r\n");
}

#[test]
fn sismember_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.sismember("key", "member");

    check_result(cmd.into(), b"SISMEMBER key member\r\n");
}

#[test]
fn smembers_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.smembers("key");

    check_result(cmd.into(), b"SMEMBERS key\r\n");
}

#[test]
fn spop_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.spop("key");

    check_result(cmd.into(), b"SPOP key\r\n");
}

#[test]
fn spop_count_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.spop_count("key", 2);

    check_result(cmd.into(), b"SPOP key 2\r\n");
}

#[test]
fn srem_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.srem("key", "member");

    check_result(cmd.into(), b"SREM key member\r\n");
}

#[test]
fn msrem_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.msrem("key", vec!["member1", "member2"]);

    check_result(cmd.into(), b"SREM key member1 member2\r\n");
}

#[test]
fn strlen_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.strlen("key");

    check_result(cmd.into(), b"STRLEN key\r\n");
}

#[test]
fn ttl_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.ttl("key");

    check_result(cmd.into(), b"TTL key\r\n");
}

#[test]
fn unwatch_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.unwatch();

    check_result(cmd.into(), b"UNWATCH\r\n");
}

#[test]
fn watch_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.watch("key");

    check_result(cmd.into(), b"WATCH key\r\n");
}

#[test]
fn mwatch_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.mwatch(vec!["key1", "key2"]);

    check_result(cmd.into(), b"WATCH key1 key2\r\n");
}

#[test]
fn zadd_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.zadd("key", 4.2, "member");

    check_result(cmd.into(), b"ZADD key 4.2 member\r\n");
}

#[test]
fn zadd_binary_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.zadd_binary("key", 4.2, b"member");

    check_result(cmd.into(), b"ZADD key 4.2 member\r\n");
}

#[test]
fn zaddnx_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.zaddnx("key", 4.2, "member");

    check_result(cmd.into(), b"ZADD key NX 4.2 member\r\n");
}

#[test]
fn zaddxx_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.zaddxx("key", 4.2, "member");

    check_result(cmd.into(), b"ZADD key XX 4.2 member\r\n");
}

#[test]
fn zaddnx_ch_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.zaddnx_ch("key", 4.2, "member");

    check_result(cmd.into(), b"ZADD key NX CH 4.2 member\r\n");
}

#[test]
fn zaddxx_ch_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.zaddxx_ch("key", 4.2, "member");

    check_result(cmd.into(), b"ZADD key XX CH 4.2 member\r\n");
}

#[test]
fn zcard_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.zcard("key");

    check_result(cmd.into(), b"ZCARD key\r\n");
}

#[test]
fn zcount_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.zcount("key", "-inf", 3);

    check_result(cmd.into(), b"ZCOUNT key -inf 3\r\n");
}

#[test]
fn zincrby_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.zincrby("key", 4.2, "member");

    check_result(cmd.into(), b"ZINCRBY key 4.2 member\r\n");
}

#[test]
fn zlexcount_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.zlexcount("key", "-", "[b");

    check_result(cmd.into(), b"ZLEXCOUNT key - [b\r\n");
}

#[test]
fn zrem_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.zrem("key", "member");

    check_result(cmd.into(), b"ZREM key member\r\n");
}

#[test]
fn mzrem_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.mzrem("key", vec!["member1", "member2"]);

    check_result(cmd.into(), b"ZREM key member1 member2\r\n");
}

#[test]
fn zrange_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.zrange("key", -1, 1);

    check_result(cmd.into(), b"ZRANGE key -1 1\r\n");
}

#[test]
fn zrange_with_scores_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.zrange_with_scores("key", -1, 1);

    check_result(cmd.into(), b"ZRANGE key -1 1 WITHSCORES\r\n");
}

#[test]
fn zrevrange_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.zrevrange("key", -1, 1);

    check_result(cmd.into(), b"ZREVRANGE key -1 1\r\n");
}

#[test]
fn zrevrange_with_scores_cmd_works() {
    let cmd = &mut RedisCommand::new();
    cmd.zrevrange_with_scores("key", -1, 1);

    check_result(cmd.into(), b"ZREVRANGE key -1 1 WITHSCORES\r\n");
}