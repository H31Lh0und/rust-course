fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = [
            ((42, 56), 14),
            ((27, 18), 9),
            ((81, 63), 9),
            ((144, 48), 48),
            ((35, 25), 5),
            ((121, 11), 11),
            ((99, 33), 33),
            ((77, 22), 11),
            ((55, 20), 5),
            ((91, 13), 1),
            ((132, 96), 12),
        ];

        for ((a, b), exp) in data.iter() {
            assert_eq!(*exp, gcd(*a, *b));
        }
    }
}
