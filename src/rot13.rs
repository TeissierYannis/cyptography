pub (crate) fn run(input: &str) {

    println!("Rot13");

    // create a new string
    let mut result = String::new();

    // iterate over the input string
    for c in input.chars() {
        // get the character code
        match c {
            // if it's a letter, add 13 to it
            'a'..='m' | 'A'..='M' => result.push((c as u8 + 13) as char),
            // if it's a letter, subtract 13 from it
            'n'..='z' | 'N'..='Z' => result.push((c as u8 - 13) as char),
            // otherwise, just add it
            _ => result.push(c),
        }
    }
    // print the result
    println!("{}", result);
}