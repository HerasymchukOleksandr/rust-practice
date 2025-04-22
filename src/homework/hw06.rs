#[test]
fn main() {
    let triangles = 5;
    draw_tree(triangles);
}

fn draw_tree(triangles: usize) {
    let total_height = (1..=triangles).sum::<usize>();
    let max_width = 1 + 2 * (total_height - 1);

    let mut current_star_lines: Vec<String> = Vec::new();
    
    for level in 1..=triangles {
        
        let stars = "*".repeat(1 + 2 * (level - 1));
        current_star_lines.push(center(&stars, max_width));

        // Малюємо усі рядки, які вже є
        for line in &current_star_lines {
            println!("{}", line);
        }
    }
}

fn center(s: &str, width: usize) -> String {
    let padding = (width - s.len()) / 2;
    format!("{}{}", " ".repeat(padding), s)
}
