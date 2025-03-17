fn draw_tree(triangles: u32) {
    let mut tree = String::new();
    let max_width = 2 * triangles + 1;

    for t in 0..triangles {
        let height = t + 2;
        for i in 0..height {
            let stars = 2 * i + 1;
            let spaces = (max_width - stars) / 2;
            tree.push_str(&" ".repeat(spaces));
            tree.push_str(&"*".repeat(stars));
            tree.push('\n');
        }
    }

    print!("{}", tree);
}

fn main() {
    let triangles = 5; // Количество треугольников в ёлке
    draw_tree(triangles);
}
