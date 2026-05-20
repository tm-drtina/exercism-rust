fn to_digits(mut num: u32) -> Vec<u32> {
    let mut res = Vec::new();
    while num > 0 {
        res.push(num % 10);
        num /= 10;
    }
    res
}

pub fn is_armstrong_number(num: u32) -> bool {
    let digits = to_digits(num);
    digits.iter().map(|n| n.pow(digits.len() as u32)).sum::<u32>() == num
}
