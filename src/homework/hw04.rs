#[test]
fn diamond() {
    const W: u32 = 11; 
    const H: u32 = 11; 
    let mid = H / 2; 

    for y in 0..H {
        for x in 0..W {
            let num_stars = if y <= mid {
                1 + y * 2
            } else {
                1 + (H - 1 - y) * 2
            };
            let spaces = (W - num_stars) / 2;

            let is_star = x >= spaces && x < spaces + num_stars;
            let sym = if is_star { '*' } else { ' ' };
            print!("{}", sym);
        }
        println!();
    }
}