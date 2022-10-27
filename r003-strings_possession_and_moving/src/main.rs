fn main() {
    let mut s1 = "hello".to_string();
    println!("s1: {}",s1);

    //String does not implement the copy trait, so the value to which s1 pointer was pointing has been moved to s2 pointer
    let s2 = s1;
    println!("s2: {}",s2);

    //this results in error as s1 is for now inaccessible: s1 has no value
    /*
    println!("s1.to_uppercase(): {}", s1.to_uppercase());
     */

    //if i assign a value to s1 again, now it's accessible and can be modified again
    //s2 will still have the value "hello"
    s1 = "world".to_string();

    println!("s1.to_uppercase(): {}", s1.to_uppercase());
    println!("s2: {}", s2);
}
