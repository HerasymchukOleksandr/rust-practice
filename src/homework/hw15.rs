#[test]
fn main() {
    let digits = (1..=9).collect::<Vec<_>>();

    let mut count = 0;

    for &m in &digits {
        for &u in &digits {
            if u == m { continue; }
            for &x in &digits {
                if x == m || x == u { continue; }
                for &a in &digits {
                    if [m, u, x].contains(&a) { continue; }

                    let muxa = 1000 * m + 100 * u + 10 * x + a;
                    let product = muxa * a;

                    let s = (product / 1000) % 10;
                    let l = (product / 100) % 10;
                    let o = (product / 10) % 10;
                    let n = product % 10;

                    let mut used = vec![m, u, x, a, s, l, o, n];
                    used.sort();
                    used.dedup();

                    if used.len() == 8 && product < 10_000 {
                        println!("  {}{}{}{}", m, u, x, a);
                        println!("×     {}", a);
                        println!("-------");
                        println!("  {}{}{}{}", s, l, o, n);
                        println!();
                        count += 1;
                    }
                }
            }
        }
    }

    println!("Кількість рішень: {}", count);
}
