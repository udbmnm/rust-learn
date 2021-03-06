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
    let apple = "ð";
    let orange = "æ©";
    println!(
        "ð len={},size={} æ© len={},size={}",
        apple.len(),
        std::mem::size_of_val(&apple),
        orange.len(),
        std::mem::size_of_val(&orange)
    );

    //1. Rust ä¸­æ¯ä¸ä¸ªå¼é½ æä¸åªæ ä¸ä¸ªææè(åé)
    //2. å½ææè(åé)ç¦»å¼ä½ç¨åèå´æ¶ï¼è¿ä¸ªå¼å°è¢«ä¸¢å¼(free)

    {
        // s å¨è¿éæ æï¼å®å°æªå£°æ
        let s = "ð"; // ä»æ­¤å¤èµ·ï¼s æ¯ææç

        // ä½¿ç¨ s
    } // æ­¤ä½ç¨åå·²ç»æï¼sä¸åææ
      // println!("s={}", s); // error: `s` does not live long enough
    let s1 = String::from("ð°");
    let s2 = s1;
    // println!("s1={},s2={}", s1,s2); // error: `s1` does not live long enough
    //s1ææèç¦»å¼ä½ç¨åï¼s1è¢«ä¸¢å¼
    //Rust æ°¸è¿ä¹ä¸ä¼èªå¨åå»ºæ°æ®ç âæ·±æ·è´â
    let s3 = s2.clone();
    println!("s3={}", s3);
    //ä»»ä½åºæ¬ç±»åçç»åå¯ä»¥æ¯ Copy çï¼ä¸éè¦åéåå­ææç§å½¢å¼èµæºçç±»åæ¯ Copy ç
    let x = 1;
    let y = x;
    println!("x={},y={}", x, y); //ä¾ç¶å¨æ ä¸
}

fn sum(a: i32, b: i32) -> i32 {
    let c = a + b;
    c
}
