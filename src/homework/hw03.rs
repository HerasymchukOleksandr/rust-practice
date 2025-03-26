#[test]
fn envelope (){
    const W: u32 =30;
    const H: u32 = 15;

    let k: f32 = W as f32 / H as f32;
    for y in 0..H {
        for x in 0..W {
            let is_hor = y ==0 || y == H-1;
            let is_ver = x ==0 || x == W-1;
            let is_diag =x==(y as f32 * k) as u32;
            let is_diag2 = (y as f32 * k) as u32==W-1-x;
            let is_show = is_hor || is_ver || is_diag || is_diag2;

            let sym = if is_show {'*'} else {' '};
            print!("{}", sym);
        }
        println!();
    }
}