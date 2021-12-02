use std::num::FpCategory::Nan;

pub(crate) fn run(cypher: &str, key: &str, exclude: char) -> String {

    // playfair cipher
    let mut cypher_split = split_into_blocks(&to_uppercase(&remove_spaces(&cypher)), 2);
    let matrix = generate_key_table(&key, exclude);

    println!("{:?}", cypher_split);

    decrypt(&mut cypher_split, &matrix);

    // TODO DECRYPT

    return "".to_string();
}

fn decrypt(cipher_splited: &mut Vec<String>, matrix : &[[char; 5]; 5]) {
    let mut i: u8 = 0;

    for i in 0..cipher_splited.len(){
        // TODO
    }

    println!("{}", cipher_splited.join(""));
}

fn split_into_blocks(text: &str, block_size: usize) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut block = String::new();
    let mut was_x : char = ' ';

    for c in text.chars() {
        if block.len() == block_size {
            blocks.push(block);
            block = String::new();
        }
        if block.contains(c) {
            block.push('X');
            was_x = c;
        } else {
            if was_x != ' ' {
                block.push(was_x);
                was_x = ' ';
            }
            block.push(c);
        }
    }
    blocks.push(block);

    blocks
}

fn remove_spaces(text: &str) -> String {
    let mut new_text = String::new();
    for c in text.chars() {
        if c != ' ' {
            new_text.push(c);
        }
    }
    new_text
}

fn to_uppercase(text: &str) -> String {
    let mut new_text = String::new();
    for c in text.chars() {
        new_text.push(c.to_uppercase().next().unwrap());
    }
    new_text
}

fn generate_key_table(key: &str, exclude: char) -> [[char; 5]; 5] {
    let mut matrix: [[char; 5]; 5] = [['a'; 5]; 5];

    let mut j = 0;
    for i in 0..key.len() {
        if i % 5 == 0 && i != 0 {
            j += 1;
        }
        matrix[j][i % 5] = key.chars().nth(i).unwrap();
    }

    for i in key.len()..25 {
        if i % 5 == 0 {
            j += 1;
        }
        get_next_letter(&mut matrix, exclude);
    }

    return matrix;
}
fn get_next_letter(matrix: &mut [[char; 5]; 5], exclude: char) {
    let mut letter: u8 = 65; // a - end = 122
    let mut continue_loop = true;

    for i in 0..5 {
        let mut j = 0;
        while j < 5 {
            if matrix[i][j] == 'a' {
                continue_loop = true;
                // if letter is more than Z stop
                if letter == 91 {
                    break;
                    // if letter is excluded
                } else if letter == 'W' as u8 {
                    letter += 1;
                    continue_loop = false;
                }
                // if letter is already in matrix
                for k in 0..5 {
                    if matrix[k].contains(&(letter as char)) {
                        letter += 1;
                        continue_loop = false;
                    }
                }

                // fill matrix
                if continue_loop {
                    matrix[i][j] = letter as char;
                    letter += 1;
                    j += 1;
                } else {
                    if j != 0 {
                        j -= 1;
                    }
                }
            } else {
                j += 1;
            }
        }
    }
}