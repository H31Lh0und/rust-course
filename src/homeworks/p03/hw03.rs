const WIDTH: usize = 30;
const HEIGHT: usize = 12;

fn main() {
    let mut output = String::new();
    
    for y in 0..=HEIGHT {
        for x in 0..=WIDTH {
            if y == 0 || y == HEIGHT {
                output.push('*');
            } else if x == 0 || x == WIDTH {
                output.push('*');
            } else if x == y || x == WIDTH - y {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }
    
    print!("{}", output);
}
