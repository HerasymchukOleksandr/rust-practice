fn is_prime(n: &u32) -> bool {
    if *n < 2 {
        return false;
    }
    if *n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let limit = (*n as f64).sqrt() as u32;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
#[test]
fn test_is_prime() {
    let test_data = [
        (0, false),
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (100, false),
        (10006, true),
    ];
    
        test_data.iter().for_each(|(n, expected)| {
        let result = is_prime(n);
        println!("Checking {} → Result: {}, Expected: {}", n, result, expected);

        assert_eq!(result, *expected);
    });
}
