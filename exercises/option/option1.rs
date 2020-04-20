// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    print_number(Option::from(13));
    print_number(Option::from(99));

    let mut numbers: [Option<u16>; 5] = Default::default();
    for iter in 0..5 {
        let number_to_add: u16 = { ((iter * 5) + 2) / (4 * 16) };
        numbers[usize::from(iter)] = Option::from(number_to_add);
    }
}
