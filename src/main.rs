fn main() {

    let num:u16 = 255;
    println!("num={}",num);
    println!("Hello, world!");
    let s1 = String::from("hello");
    let s3 = takes_ownership(s1);
    println!("s3 {:?}", s3); // error: value borrowed here after move

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
fn takes_ownership(some_string: String)->String { // some_string 进入作用域
    println!("takes_ownership {}", some_string);
    some_string
} //