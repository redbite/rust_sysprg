use r004_isbn_verifier_traits::Isbn;

fn main() {
    println!("Is valid? {}", "3-598-21508-8".to_string().is_valid_isbn()); //.to_string() as the trait is implemented for String
}