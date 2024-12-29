fn main() {
  ph();
  println!("==================");
  num();
  println!("==================");
  // 转义
  println!("原始内容,反斜杠双引号：\\\"Hello, Rust!\"\r覆盖开头:");
  println!( r####"特殊字符还可以这么处理:"原始内容,反斜杠双引号：\\\"Hello, Rust!\"\r覆盖开头
"# and "##,当其内部存在"###时,外围用更多#包裹以区分"####);
  std::process::exit(0);
}
// placeholder
fn ph() {
  println!("number:{} bool:{} String:{} char:{}", 2, true, "ABC", 'A');
  // 元组和调试输出
  let tuple = (1, "Rust", true);
  // println!("调试输出：{}", tuple); // 使用 {:?} 输出元组 {}不行
  println!("调试输出：{:?}", tuple); // 使用 {:?} 输出元组 {}不行
  println!("调试输出：{:?}", (tuple, 42)); // 使用 {:?} 输出元组
  println!("美化调试输出：{:#?}", (tuple, 42)); // 美化的嵌套调试输出
  // format!("Hello, {:#?}!", (tuple, 42))  // format!于println!处理类似，但它不会打印到控制台，而是返回一个格式化后的字符串。
  println!(
    "number:{:#?} bool:{:#?} String:{:#?} char:{:#?} Unicode：{}",
    2, true, "ABC😊", 'A','\u{1F600}'
  );
  let array = [1, 2, 3];
  println!("数组：{:?}", array);
  let map = std::collections::HashMap::from([(1, "one"), (2, "two")]);
  println!("哈希表：{:?}", map);
  // 使用命名参数
  println!(
    "{language} 是一种 {feature} 的语言。",
    language = "Rust",
    feature = "高性能"
  );
}
fn num() {
  println!("浮点数：{:.2}", 3.141592); // 保留两位小数
  println!("填充宽度：{:>5}", 42); // 右对齐，宽度5
  println!("左对齐：{:<5}", 42); // 左对齐，宽度5
  println!("用0填充：{:0>5}", 42); // 用 0 填充，宽度5
  println!("十六进制：0x{:x}", 255); // 十六进制
  println!("八进制：0o{:o}", 255); // 八进制
  println!("二进制：0b{:b}", 255); // 二进制
}
