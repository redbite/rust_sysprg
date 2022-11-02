use std::io;

fn main() -> io::Result<()>{ //io::Result is used to
    //ISBN as string input
    let mut isbn = String::new();
    let stdin = io::stdin();
    //reads the input and stores the value in the isbn variable
    stdin.read_line(&mut isbn)?;
    println!("input {} ", isbn);
    isbn= isbn.trim().parse().unwrap(); // trimming newline character

    //iterating each char of the string isbn
    let mut count =10;
    println!("isbn has {} digits",count);

    let mut sum;
    let mut total:i32=0;
    for c in isbn.chars() {
        let mut c_number = c as i32 - 0x30;
        if c != '-' {
            if c == 'X' {
                println!("X is 10");
                c_number = 10;
            }
            sum = c_number * (count as i32);
            total += sum;
            println!("c={} x {}  | sum={}  | total={}", c_number, count as i32, sum, total);
            count = count -1;
        }
    }
    // isbn is valid if total mod 11 == 0
    if ((total % 11) + 11) % 11 == 0 {
        println!("{} is a valid ISBN.",isbn);
    }else{
        println!("{} is NOT a valid ISBN.",isbn);
    }

    Ok(()) //return
}
