use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = vec![];
    io::stdin().read_to_end(&mut buffer)?;

    let entropy = entropy(&buffer);

    println!("{}", entropy);

    Ok(())
}

fn entropy(buffer: &Vec<u8>) -> f64 {
    let pools = vec![
        "abcdefghijklmnopqrstuvwxyz",
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        "0123456789",
        "~`!@#$%^&*()_-+={}|[]\\:\";'<>,./?",
        " \t\r\n",
    ];

    let member_pools = pools.iter().filter(|pool| has_members(buffer, pool));
    let pool_size: usize = member_pools.map(|pool| pool.len()).sum();

    if buffer.len() == 0 {
        0.0
    } else {
        buffer.len() as f64 * (pool_size as f64).log2()
    }
}

fn has_members(buffer: &Vec<u8>, pool: &str) -> bool {
    buffer.iter().any(|byte| pool.contains(*byte as char))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strings_have_correct_entropies() {
        let cases = [
            ("", 0.0),
            ("abc", 3.0 * 26.0_f64.log2()),
            ("ABC", 3.0 * 26.0_f64.log2()),
            ("aBC", 3.0 * 52.0_f64.log2()),
            ("aB1", 3.0 * 62.0_f64.log2()),
            ("aB1.", 4.0 * 94.0_f64.log2()),
            ("()&*", 4.0 * 32.0_f64.log2()),
            ("123", 3.0 * 10.0_f64.log2()),
            ("12-", 3.0 * 42.0_f64.log2()),
            ("ab@", 3.0 * 58.0_f64.log2()),
            ("AB@", 3.0 * 58.0_f64.log2()),
            ("\t\n\t", 3.0 * 4.0_f64.log2()),
            ("AB\r", 3.0 * 30.0_f64.log2()),
        ];

        for (input, expected) in cases {
            assert!(entropy(&input.as_bytes().to_vec()) == expected)
        }
    }
}
