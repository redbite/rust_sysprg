use std::io::stdin;
/*
    Input di un numero da tastiera
 */
fn main() {
    println!("Enter a number: ");

    // se non metto ln, non stampa niente perchè l'argomento della print va sul buffer.
    // Quindi se non metto ln, flusho (il flush, da descrizione, fa sì che gli elementi finiscono nella giusta destinazione
    /*
    print!("Enter a number: ");
    stdout().flush().unwrap();
    */

    // ERROR Cannot borrow as mutable
    /*let inputBuffer:String = String::new();
    stdin().read_line(&mut inputBuffer);*/

    //snake case required: inputBuffer --> input_buffer
    /*let mut inputBuffer:String = String::new();
    stdin().read_line(&mut inputBuffer);*/

    let mut input_buffer:String = String::new();
    // .expect("Input not possible"); to debug
    stdin().read_line(&mut input_buffer).expect("Input not possible");
    //println!("input is {}",input_buffer); // funziona lo stesso ?

    //si deve parsare e unwrappare (Prima è un contenitore) il contenuto
    //unwrap è bloccante su errori, invece unwrap_or ritorna -1 in caso di errore
    //senza trim dà errori
    /*
    let i:i32 = input_buffer.trim().parse().unwrap_or(-1); //let i = input_buffer.parse::<i32>().unwrap();
    println!("Integer: {:?}",i);
    */

    //si può fare match per distinguere le casistiche differenti
    let i:i32= match input_buffer.trim().parse(){
        Ok(i)=>i,
        Err(e) => {
            eprintln!("{}: you did not enter a number",e);
            -1
        }
    };
    println!("Integer: {:?}",i);
}
