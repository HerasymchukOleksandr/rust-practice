use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..=99)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (usize, i32, i32) {
    let mut min_sum = i32::MAX;
    let mut index = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            index = i;
        }
    }

    (index, data[index], data[index + 1])
}

fn print_vector_with_min_pair(data: &[i32]) {
    let (i, a, b) = min_adjacent_sum(data);

    // Вивід індексів
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:>3}.", i);
    }
    println!();

    // Вивід даних
    print!("data:   [");
    for (j, val) in data.iter().enumerate() {
        print!("{:>2}", val);
        if j != data.len() - 1 {
            print!(", ");
        }
    }
    println!("]");

    // Підкреслення
    print!("indexes:");
    for j in 0..data.len() {
        if j == i {
            print!("\\__");
        } else if j == i + 1 {
            print!(" __/");
        } else {
            print!("    ");
        }
    }
    println!();

    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        a,
        b,
        a + b,
        i,
        i + 1
    );
}
#[test]
fn main() {
    for _ in 0..5 {
        let data = gen_random_vector(20);
        print_vector_with_min_pair(&data);
        println!();
    }
}

