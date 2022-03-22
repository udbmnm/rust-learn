use rand::Rng;
use rust_learn::echo;
use rust_learn::echo::learn::say;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("secret_number={}", secret_number);
    echo::play::doing();
    say();
    let text = "\

  Rust is a systems programming language, that is compiled to LLVM.
  It is a compiled language, which means that it is compiled to machine code.
  However, it is not a compiled language, which means that it is not compiled to machine code.
  But it is a compiled language, which means that it is compiled to machine code.
  ";

    let records = text.lines();

    for (i, record) in records.enumerate() {
        if i == 1 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record.split(',').map(|s| s.trim()).collect();

        if cfg!(debug_assertions) {
            eprintln!("debug{:?} -> {:?}", fields, record)
        }

        let name = fields[0];

        if let Ok(length) = name.parse::<f32>() {
            println!("name{}->length{}", name, length);
        }

        println!("{}", i);
    }

    let mut a: i32 = 1;
    let b = 2;
    let c = 30_i32;
    println!("a={},b={}", a, b);
    a = 4;
    println!("a={},c={}", a, c);
    // b = 3; // error: cannot assign to immutable variable `b`
    let d = sum(a, c);
    println!("d={}", d);
    let y = {
        let x = 3;
        x + 1
    };
    println!("y={}", y);
    let apple = "🍎";
    let orange = "橘";
    println!(
        "🍎 len={},size={} 橘 len={},size={}",
        apple.len(),
        std::mem::size_of_val(&apple),
        orange.len(),
        std::mem::size_of_val(&orange)
    );

    //1. Rust 中每一个值都 有且只有 一个所有者(变量)
    //2. 当所有者(变量)离开作用域范围时，这个值将被丢弃(free)

    {
        // s 在这里无效，它尚未声明
        let s = "🍉"; // 从此处起，s 是有效的

        // 使用 s
    } // 此作用域已结束，s不再有效
      // println!("s={}", s); // error: `s` does not live long enough
    let s1 = String::from("🌰");
    let s2 = s1;
    // println!("s1={},s2={}", s1,s2); // error: `s1` does not live long enough
    //s1所有者离开作用域，s1被丢弃
    //Rust 永远也不会自动创建数据的 “深拷贝”
    let s3 = s2.clone();
    println!("s3={}", s3);
    //任何基本类型的组合可以是 Copy 的，不需要分配内存或某种形式资源的类型是 Copy 的
    let x = 1;
    let y = x;
    println!("x={},y={}", x, y); //依然在栈上
}

fn sum(a: i32, b: i32) -> i32 {
    let c = a + b;
    c
}
