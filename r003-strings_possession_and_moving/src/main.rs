fn main() {
    let mut s1 = "hello".to_string();
    println!("s1: {}",s1);

    let s2 = s1;
    println!("s2: {}",s2);

    s1 = "world".to_string();

    println!("s1.to_uppercase(): {}", s1.to_uppercase());
    println!("s2: {}", s2);
}
