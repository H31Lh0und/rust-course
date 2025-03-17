const HEIGHT: usize = 5;

fn main() {
    let mut output = String::new();
    
    for i in 0..HEIGHT {
        for _ in 0..(HEIGHT - i - 1) {
            output.push(' ');
        }
        for _ in 0..(2 * i + 1) {
            output.push('*');
        }
        output.push('\n');
    }
    
    for i in (0..HEIGHT - 1).rev() {
        for _ in 0..(HEIGHT - i - 1) {
            output.push(' ');
        }
        for _ in 0..(2 * i + 1) {
            output.push('*');
        }
        output.push('\n');
    }
    
    print!("{}", output);
}
