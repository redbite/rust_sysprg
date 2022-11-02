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
    for c in isbn.chars() {
        let mut c_number = c as i32 - 0x30;
        if c != '-' {
            if c == 'X' {
                println!("X is 10");
                c_number = 10;
            }
            sum = c_number * (count as i32);
            println!("c={}, c_number={}  |  count={}  | sum={}", c, c_number, count as i32, sum);
            count = count -1;
        }
    }



    Ok(()) //return
}
