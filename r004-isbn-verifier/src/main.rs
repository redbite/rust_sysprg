use std::io;

fn main() -> io::Result<()>{ //io::Result is used to
    //ISBN as string input
    let mut isbn = String::new();
    let stdin = io::stdin();
    //reads the input and stores the value in the isbn variable
    stdin.read_line(&mut isbn)?;
    println!("input {} ", isbn);




    Ok(()) //return
}
