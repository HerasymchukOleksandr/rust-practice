

/*
3. Чи завжди можливо всі кораблі забезпечити однаковою кількість грузу?
Ні, не завжди. Щоб усі кораблі мали однакову кількість вантажу, сума всіх вантажів повинна ділитися на кількість кораблів без остачі.
4. як буде виглядати сігнатура в іншому випадку?
Замість usize, потрібно використати Option<usize> щоб коректно обробляти випадки, коли розв’язку не існує.
*/


use rand::Rng;

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let avg = rng.gen_range(10..50);
    let mut shipments = vec![avg; n];

    for _ in 0..n / 2 {
        let i = rng.gen_range(0..n);
        let j = rng.gen_range(0..n);
        if i != j && shipments[i] > 1 {
            let shift = rng.gen_range(1..=shipments[i].min(10));
            shipments[i] -= shift;
            shipments[j] += shift;
        }
    }

    shipments
}

fn  count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len();
    if total as usize % n != 0 {
        return None;
    }

    let avg = total / n as u32;

 
    println!("{:?}", shipments);
    println!("total   = {} = {}", shipments.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" + "), total);
    println!("average = {}", avg);
    
    print!("[");
    for (i, v) in shipments.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", v);
    }
    println!("]");

    let mut balance = 0i32;
    let mut moves = 0usize;

    let mut shifts: Vec<String> = vec![];

    for &x in shipments {
        let diff = x as i32 - avg as i32;
        balance += diff;
        moves += diff.abs() as usize;

        let s = if diff == 0 {
            "   ".to_string()
        } else if diff > 0 {
            format!("{:>+3}", -diff)
        } else {
            format!("{:>+3}", diff.abs())
        };

        shifts.push(s);
    }

    for shift in shifts {
        print!("{:<4}", shift);
    }
    println!("\n");

    println!("answer = {}", moves / 2);
    println!("------------------------------------");
    Some(moves / 2)
    
}
#[test]
fn main() {
    for _ in 0..5 {
        let shipments = gen_shipments(10);
        match  count_permutation(&shipments) {
            Some(_) => println!(),
            None => println!("It's not possible to balance the cargo!\n"),

        }
        
        let invalid_shipments = vec![1, 1, 1, 6];
        match count_permutation(&invalid_shipments) {
            Some(_) => println!(),
            None => println!("It's not possible to balance the cargo!\n"),
    }
}
}


