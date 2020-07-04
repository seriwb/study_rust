fn main() {
  println!("Hello, world!");

  let name = "seri";
  let name2: &str = "Your name";

  let age = 10;
  let age2: i64 = 99;

  // 浮動小数点を扱う
  let x = 100.234;
  println!("x is {}", x);
  let x: f64 = 100.234;
  println!("x is {}", x);

  // 論理値型を扱う bool
  let f = true;
  println!("f is {}", f);

  let cat: char = '😺';

  // String型の使用
  let s = String::from("Hello Rust world.");
  println!("{}", s);

  let s1 = String::from("Hey");
  let s2 = String::from("Rust");
  let s3 = String::from("world.");
  // let s = s1 + " " + &s2 + " " + &s3;
  let s = format!("{} {} {}", s1, s2, s3);  // s1を上で使っているとこれはエラーになった
  println!("{}", s);
}

fn add(x: i32, y: i32) -> i32 {
  x + y
}
