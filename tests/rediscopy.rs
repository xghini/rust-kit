//tokio::test宏期待Result<()>返回, 另外tokio::test会干扰补全哪怕显式声明也没用
#[tokio::test]
async fn test() -> RedisResult<()> {
  t1().await?;
  // test_advanced_lua_set().await?;
  Ok(())
}
async fn t1() -> RedisResult<()> {
  let mut conn = recn().await?;
  // 这里set返回值哪怕不用,也需要显式接收声明类型,否则会报错
  // 方式1
  // conn.set(KEY, VALUE).await?; //warn
  let _: String = conn.set("t0", "v0").await?; //correct

  // 方式2 直接使用类型标注而不存储结果
  // conn.set::<String>(KEY, VALUE).await?; //error
  // conn.set::<&str, &str, String>(KEY, VALUE).await?; //correct

  let _: String = conn.set_ex("t1", "v1", 10).await?;
  let ttl: i64 = conn.ttl("t1").await?;
  println!("超时时间:{}", ttl);

  let res1: Option<String> = conn.get("t00").await?;
  // get成功的话获取Some("value"),是Option<&str>类型,即一个Option枚举，表示有值或者可能没值的场景
  println!("获取的值是Option: {:?}", res1);
  let v0 = res1.unwrap_or_default();
  // let v0 = if let Some(v) = res1 {v} else {Default::default()};
  println!("对Option进行提取: {:?}", v0);
  let res2: i64 = conn.del("t0").await?;
  println!("删除的键数量: {:?}", res2);

  // 当你不知道返回类型时,随便定义一个,看报错可以判断
  // 这里hgetall可以不用Option wrap,若没有会返回空的HashMap,{}
  let res3: HashMap<String, String> = conn.hgetall("user:admin@xship.top").await?;
  println!("hset的结果: {:?}", res3);

  /* 对返回类型的定义,还能形成不同的返回结果?
   实际上，当你使用不同的类型声明时，Redis 客户端库会进行不同的类型转换处理,这涉及到了 Rust 的 trait 系统和类型推断
    在 Redis 客户端库中，hgetall 方法的返回值很可能是通过类似这样的泛型实现的：

  */
  let res4: Option<HashMap<String, String>> = conn.hgetall("user:admin@xship.top").await?;
  println!("hset的结果: {:?}", res4);

  Ok(())
}

#[tokio::test]
async fn test_advanced_lua_set() -> RedisResult<()> {
  let mut conn = recn().await?;
  let script = redis::Script::new(
    r#"
      -- 检查键是否已存在
      local exists = redis.call('EXISTS', KEYS[1])
      
      if exists == 0 then
          -- 键不存在，设置新值和过期时间
          redis.call('SETEX', KEYS[1], ARGV[2], ARGV[1])
          return 'SET_NEW'
      else
          -- 键已存在，获取当前 TTL
          local ttl = redis.call('TTL', KEYS[1])
          -- 只有当剩余时间小于新的过期时间时才更新
          if ttl < tonumber(ARGV[2]) then
              redis.call('SETEX', KEYS[1], ARGV[2], ARGV[1])
              return 'UPDATED'
          end
          return 'KEPT_OLD'
      end
  "#,
  );
  let result: String = script
    .key("t0")
    .arg("v0")
    .arg(300)
    .invoke_async(&mut conn)
    .await?;
  println!("操作结果: {}", result);
  Ok(())
}
/*
1.学习redis使用
2.学习类型(错误)处理
RedisResult<()> 表示函数返回一个结果，可能是成功 (Ok) 或失败 (Err),异步函数使用?操作符需要返回ErrResult类型
3.学习OnceLock存储可复用对象
4.注释 1.单行注释 (//) 2.块注释 (/* */) 3.文档注释[文档生成]（///用于标注函数、结构体等、//!用于模块或crate级别常于文件开头提供整体说明、/** */）
内部文档注释//! /*! */或用于描述"包含它的东西"，而外部文档注释/// /** */用于描述"跟在它后面的东西"。

use redis::AsyncCommands 引入了一个 trait，它为 Redis 连接提供了异步操作方法
这些方法来自 AsyncCommands trait
conn.set(key, value).await
conn.get(key).await
conn.del(key).await
*/
use redis::{aio::MultiplexedConnection, AsyncCommands, RedisResult};
use std::{collections::HashMap, sync::OnceLock};
// static const 需要满足线程安全,都得显式type不依靠推断.
/*
static [mut]：
1.有固定的内存地址,分配在程序的静态存储区域（通常是全局内存）。即使它是不可变的，它依然占用一个全局内存位置，程序运行时可以通过这个地址访问它。
2.可以用于运行时初始化的值（只要它满足 'static 生命周期）。可以存储复杂的结构，比如引用、数组、全局对象等。
const：
1.它没有固定的存储位置，不会占用全局内存,值会被内联到使用它的地方。编译器会直接将 const 的值嵌入到代码中（类似于宏展开的效果），这意味着它本质上是“按值使用”，而不是“按地址引用”。
2.只能用于编译时已知的值（常量表达式）。初始化时的值必须是完全静态的，无法依赖任何运行时后处理(lazy)的值。
*/
// 全局存储 Redis 客户端
static REDIS_CLIENT: OnceLock<redis::Client> = OnceLock::new();
/*
Rust 中的 static 变量默认是 不可变 的，这意味着你不能直接通过 REDIS_CLIENT 调用其方法，因为这些方法（比如 get_or_init）需要使用 可变引用 来操作内部数据
通过unsafe或封装更高层次的线程安全工具来操作 static。比如函数封装来避免直接访问static。
为什么函数封装可以解决问题？
函数封装会将所有的静态访问隐藏在函数内部，OnceLock 自身通过内部的 UnsafeCell 实现了线程安全的惰性初始化（get_or_init 本身是安全的）
那mut可以吗?
还是不可以，因为 Rust 的 static mut 的使用是非常受限的。尽管 static mut 声明允许全局变量是可变的，但你仍然会遇到以下几个问题：
1.访问 static mut 时需要 unsafe 块,Rust 强制要求你在访问或修改 static mut 时使用 unsafe 块来显式声明可能存在的风险
2.即使unsafe包裹,多线程访问时可能导致未定义行为,static mut 不会自动加锁或进行同步控制，你需要手动确保线程安全
*/
fn init_redis_client() -> &'static redis::Client {
  // || { ... } 表示一个没有参数的闭包
  REDIS_CLIENT.get_or_init(|| {
    redis::Client::open("redis://127.0.0.1:6379").expect("Failed to create Redis client")
  })
}
// 获取 Redis 连接，复用全局客户端
// 原redis::aio::Connection已被弃用，应使用redis::aio::MultiplexedConnection
async fn recn() -> RedisResult<MultiplexedConnection> {
  let client = init_redis_client();
  let conn = client.get_multiplexed_async_connection().await?;
  Ok(conn)
}
