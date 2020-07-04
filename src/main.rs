fn main1() {
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
  let s = format!("{} {} {}", s1, s2, s3); // s1を上で使っているとこれはエラーになった
  println!("{}", s);

  // タプルの例
  let t = ("seri", 17);
  let t2 = (name, age);
  println!("name is {} age {}", t.0, t.1);
  println!("name is {} age {}", t2.0, t2.1);

  // 配列型
  let a = ["春", "夏", "秋", "冬"];
  println!("最初の季節 {}", a[0]);
  println!("最後の季節 {}", a[3]);
}

fn add(x: i32, y: i32) -> i32 {
  x + y
}

// 参照と借用の確認用
fn main2() {
  // 移動
  let x = 100;
  // let x = String::from("Hello"); // こっちだとprintln!の借用でエラー
  let y = x; // ここはコピーされている
  println!("x is {}", x);
  println!("y is {}", y);

  // 借用の理解用
  let x = String::from("Hello");
  let len = string_length(&x); // 参照
  println!("{}'s len is {}", x, len);
}

// fn string_length(s: String) -> usize { // これだと移動になるので、もとの値が借用できない
fn string_length(s: &String) -> usize {
  let length = s.len();
  length
}

// 束縛の確認用
fn main3() {
  let mut x = 100;
  x = x + 100;
  println!("{}", x);

  // 宣言と代入を分けるのはOK
  let y: &str;
  y = "10";

  // シャドーイング
  let x = 100;
  let x = 300;
  println!("{}", x);
}

// スコープの確認用
fn main4() {
  let x = 100;
  println!("x is {}", x);
  {
    let x = 200;
    println!("x is {}", x);
  }
  println!("x is {}", x);
}

fn test(x: i32) -> i32 {
  let mut ans = x;
  if x < 0 {
    ans = 0;
  }
  if x > 100 {
    ans = 100;
  }
  ans
}

fn test2(x: i32) -> i32 {
  // ifは式に使える
  let ans = if x < 0 {
    0
  } else if x > 100 {
    100
  } else {
    x
  };
  ans
}

// 構造体のスコープ例
struct Sample {
  x: i32,
}
impl Sample {
  fn new(x: i32) -> Sample {
    Sample { x: x }
  }
  fn inc(&self) -> i32 {
    self.x + 1
  }
  fn add(&self, x: i32) -> i32 {
    self.x + x
  }
}

fn main5() {
  let a = Sample::new(10);
  let ans = a.inc();
  println!("ans is {}", ans);
  let ans = a.add(20);
  println!("ans is {}", ans);
}

// クロージャのスコープ例
fn main() {
  let num = 10;
  let add_one = |x:i32| { num + x };
  let add_two = |x, y| { x + y };

  let ans = add_one(1);
  println!("ans is {}", ans);
  let ans = add_two(10, 20);
  println!("ans is {}", ans);
}