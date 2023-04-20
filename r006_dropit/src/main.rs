use r006_dropit::DropIt;

fn main(){
    let mut a = DropIt::new("this is a message".to_string());
    println!("We are still in the main fn and s is {}", a.get_s());
    a.set_s_to_void();
}