use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX;
    let mut min_index = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    (min_sum, min_index, min_index + 1)
}

fn print_result(data: &[i32], min_sum: i32, min_index1: usize, min_index2: usize) {
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:2}. ", i);
    }
    println!();

    print!("data:   ");
    for &n in data {
        print!("{:2} ", n);
    }
    println!();

    print!("indexes: ");
    for i in 0..min_index1 {
        print!("    ");
    }
    print!("\\__ __/ ");
    for i in min_index2 + 1..data.len() {
        print!("    ");
    }
    println!();

    println!("min adjacent sum={}+{}={} at indexes:{},{}", 
        data[min_index1], 
        data[min_index2], 
        min_sum, 
        min_index1, 
        min_index2
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = [
            32, 76, 58, 47, 99, 61, 81, 35, 59, 74, 83, 52, 44, 64, 28, 95, 42, 81, 92, 53
        ];
        let (min_sum, min_index1, min_index2) = min_adjacent_sum(&data);
        print_result(&data, min_sum, min_index1, min_index2);

        let data2 = [
            84, 67, 25, 55, 38, 91, 72, 14, 56, 76, 32, 87, 60, 45, 34, 89, 24, 42, 78, 65
        ];
        let (min_sum2, min_index1_2, min_index2_2) = min_adjacent_sum(&data2);
        print_result(&data2, min_sum2, min_index1_2, min_index2_2);

        let data3 = [
            50, 22, 84, 79, 62, 33, 41, 54, 29, 57, 72, 92, 88, 41, 35, 69, 18, 64, 46, 71
        ];
        let (min_sum3, min_index1_3, min_index2_3) = min_adjacent_sum(&data3);
        print_result(&data3, min_sum3, min_index1_3, min_index2_3);

        let data4 = [
            15, 72, 55, 63, 93, 74, 48, 35, 94, 26, 47, 56, 91, 85, 72, 38, 16, 79, 53, 88
        ];
        let (min_sum4, min_index1_4, min_index2_4) = min_adjacent_sum(&data4);
        print_result(&data4, min_sum4, min_index1_4, min_index2_4);
    }
}
