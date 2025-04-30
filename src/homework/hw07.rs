fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else if c.is_lowercase() {
                c.to_uppercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect()
}
#[test]
fn test() {
    let data =
        [
            ("Hello", "hELLO"),
            ("Привет", "пРИВЕТ"),
        ];


    data.iter().for_each(|(a, b)| {
        let result_ab = invert_the_case(a.to_string());
        let result_ba = invert_the_case(b.to_string());

        println!("Input: {}, Output: {}", a, result_ab);
        println!("Input: {}, Output: {}", b, result_ba);

        assert_eq!(result_ab, b.to_string());
        assert_eq!(result_ba, a.to_string());
    });

}
