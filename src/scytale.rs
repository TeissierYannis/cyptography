// Scytale transposition
// This is a simple Scytale transposition cipher.
pub(crate) fn run(encrypted: &str) {
    println!("Scytale transposition");
    // Decrypt the message
    decrypt(&encrypted);
}

/**
 * Decrypt the message
 * @param encrypted The encrypted message
 */
fn decrypt(encrypted: &str) {
    for key in 1..20 {
        let matrix = get_matrix(key, &encrypted);

        for j in 0..matrix[0].len() {
            for i in 0..matrix.len() {
                print!("{}", matrix[i][j]);
            }
        }
        print!(" : {} \n", key);
    }
}

/**
 * Get the matrix for the given key
 * @param key The key
 * @param encrypted The encrypted message
 * @return The matrix
 */
fn get_matrix(key: u8, encrypted: &str) -> Vec<Vec<char>> {
    let size: usize = encrypted.len(); // SIZE
    let l: usize = ((size / key as usize) as f32).ceil() as usize; // nb ruban tours
    let n: usize = ((size / l) as f32).ceil() as usize; // nb lettres par tour

    let ca: Vec<char> = encrypted.chars().collect();
    let mut matrix = vec![vec!['a'; n]; l];

    for i in 0..n {
        for j in 0..l {
            matrix[j][i] = ca[(j * n) + i];
        }
    }
    return matrix;
}