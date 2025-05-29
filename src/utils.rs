pub fn to_ternary(mut n: usize) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let mut digits = Vec::new();
    while n > 0 {
        digits.push((n % 3).to_string());
        n /= 3;
    }
    digits.reverse();
    digits.concat()
}

pub fn from_ternary(s: &str) -> usize {
    s.chars().rev().enumerate().map(|(i, c)| {
        let digit = c.to_digit(10).unwrap();
        digit as usize * 3_usize.pow(i as u32)
    }).sum()
}