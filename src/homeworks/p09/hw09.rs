fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s;
    }

    // Знайдемо ефективний зсув, з урахуванням довжини рядка
    let n = ((n % len as isize) + len as isize) % len as isize;

    // Виконання зсуву
    let n = n as usize; // перетворюємо зсув на unsigned тип
    let (left, right) = s.split_at(len - n);
    format!("{}{}", right, left)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "ijklmnop".to_string();
        let shifts = [
            (0,  "ijklmnop"),
            (8,  "ijklmnop"),
            (-8, "ijklmnop"),
            (1,  "jklmnoip"),
            (2,  "opijklmn"),
            (10, "opijklmn"),
            (-1, "klmnopij"),
            (-2, "lmnopijk"),
            (-10,"lmnopijk"),
        ];

        shifts.iter().for_each(|(n, exp)| {
            assert_eq!(
                rotate(s.clone(), *n),
                exp.to_string()
            );
        });
    }
}
