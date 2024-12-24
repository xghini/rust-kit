use anyhow::Result;
use redis::AsyncCommands;
use redis::Value; // 引入 Redis Value 类型

// 创建 Redis 客户端连接
async fn create_redis_connection() -> Result<redis::aio::Connection> {
  let client = redis::Client::open("redis://127.0.0.1:6379")?;
  let conn = client.get_async_connection().await?;
  Ok(conn)
}

// 异步设置值 - 明确指定返回类型
async fn redis_set(key: &str, value: &str) -> Result<Value> {
  let mut conn = create_redis_connection().await?;
  // 使用 cmd 来明确指定返回类型
  let result = redis::cmd("SET")
    .arg(key)
    .arg(value)
    .query_async(&mut conn)
    .await?;
  Ok(result)
}

// 异步获取值
async fn redis_get(key: &str) -> Result<String> {
  let mut conn = create_redis_connection().await?;
  let value = conn.get(key).await?;
  Ok(value)
}

// 清理测试数据
async fn clean_test_key(key: &str) -> Result<()> {
  let mut conn = create_redis_connection().await?;
  let _: () = redis::cmd("DEL").arg(key).query_async(&mut conn).await?;
  Ok(())
}
// 测试设置值
#[tokio::test]
async fn test_set() {
  let test_key = "test_key_set";
  let test_value = "Ghini";
  let result = redis_set(test_key, test_value).await;
  assert!(result.is_ok(), "设置值失败: {:?}", result.err());
  println!("✓ 成功设置值: {} = {}", test_key, test_value);
}

#[cfg(test)]
mod tests {
  use super::*;

  // 测试设置值
  #[tokio::test]
  async fn test_set() {
    let test_key = "test_key_set";
    let test_value = "hello redis set";

    // 执行设置操作
    let result = redis_set(test_key, test_value).await;
    assert!(result.is_ok(), "设置值失败: {:?}", result.err());
    println!("✓ 成功设置值: {} = {}", test_key, test_value);

    // 清理测试数据
    clean_test_key(test_key).await.expect("清理测试数据失败");
    println!("✓ 测试数据已清理");
  }

  // 测试获取值
  #[tokio::test]
  async fn test_get() {
    let test_key = "test_key_get";
    let test_value = "hello redis get";

    // 先设置测试数据
    redis_set(test_key, test_value)
      .await
      .expect("设置测试数据失败");
    println!("✓ 测试数据已设置");

    // 测试获取操作
    let result = redis_get(test_key).await;
    assert!(result.is_ok(), "获取值失败: {:?}", result.err());

    let value = result.unwrap();
    assert_eq!(
      value, test_value,
      "获取的值 '{}' 与设置的值 '{}' 不匹配",
      value, test_value
    );
    println!("✓ 成功获取值: {}", value);

    // 清理测试数据
    clean_test_key(test_key).await.expect("清理测试数据失败");
    println!("✓ 测试数据已清理");
  }

  // 测试设置后立即获取
  #[tokio::test]
  async fn test_set_get() {
    let test_key = "test_key_set_get";
    let test_value = "hello redis set and get";

    println!("开始设置和获取测试");

    // 设置值
    redis_set(test_key, test_value).await.expect("设置值失败");
    println!("✓ 值已设置");

    // 获取并验证值
    let value = redis_get(test_key).await.expect("获取值失败");
    assert_eq!(value, test_value, "值不匹配");
    println!("✓ 获取的值正确: {}", value);

    // 清理测试数据
    clean_test_key(test_key).await.expect("清理测试数据失败");
    println!("✓ 测试数据已清理");
  }
}
