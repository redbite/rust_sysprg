pub trait Isbn {
    fn is_valid_isbn(&self) -> bool;
}

impl Isbn for String {
    fn is_valid_isbn(&self) -> bool {
        let mut tot = 0;
        let mut iterator_int = 10; //per scorrere

        for ch in self.chars() {
            match ch {
                '-' => continue, //to the next
                ' ' => continue,
                'x' | 'X' if iterator_int == 1 => tot += 10, //only last position (iterator_int 1)
                '0'..='9' => tot += ch.to_digit(10).unwrap() * iterator_int, //if the value is in 0,9 --> converting, else error
                _ => return false
            }

            if iterator_int == 0 { return false; } //error if the input lenght exceeds
            iterator_int -= 1;
        }
        //boolean condition that returns false or true
        //tot%11=0 and lenght=10 (interator_int = 0 after the final decrement)
        tot % 11 == 0 && iterator_int == 0
    }
}