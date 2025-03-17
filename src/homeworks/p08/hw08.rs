fn is_prime(n: &u32) -> bool {
    if *n < 2 {
        return false;
    }
    for i in 2..=(*n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        let test_data = [
            (6, false),
            (17, true),
            (20, false),
            (23, true),
            (49, false),
            (89, true),
            (99, false),
            (101, true),
        ];

        test_data.iter().for_each(|(n, prime)| {
            assert_eq!(is_prime(n), *prime);
        });
    }
}
