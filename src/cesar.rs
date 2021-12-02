pub(crate) fn run(input: &str) {
    println!("Cesar cipher");
    println!("Input: {}", input);

    for key in 0..26 {
        let output = decrypt(input, key);
        println!("Key: {}", key);
        println!("Output: {}", output);
    }
}

fn decrypt(input: &str, key: u8) -> String {
    let mut output = String::new();

    for char in input.chars() {
        let mut c = char as u8;
        c += key;
        if char.is_uppercase() {
            if c > 'Z' as u8 {
                c -= 26;
            }
        } else {
            if c > 'z' as u8 {
                c -= 26;
            }
        }
        output.push(c as char);
    }

    return output;
}