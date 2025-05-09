fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s;
    }

    let n = n.rem_euclid(len as isize) as usize;
    let split = len - n;
    format!("{}{}", &s[split..], &s[..split])
}
#[test]
fn test() {
    let s = "abcdefgh".to_string();
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];


    shifts.iter().for_each(|(n, expected)| {
        let result = rotate(s.clone(), *n);
        println!("rotate({}, {}) > {}", s, n, result);
        assert_eq!(result, expected.to_string());
    });
}
